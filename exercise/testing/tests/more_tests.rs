use testing::*;

#[test]
fn test_3() {
    assert!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) == 4);
}
