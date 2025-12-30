// Generated macro for gcdi (function)
macro_rules! Depcrate_testsgcdi {
() => {
// Module: crate::tests
// Provides: {"gcdi"}
// Dependencies: {}
# [doc = " Computes the greatest common divisor of two integers."] fn gcdi (mut a : i64 , mut b : i64) -> i64 { a = a . abs () ; b = b . abs () ; while a != 0 { let tmp = b % a ; b = a ; a = tmp ; } b }
};
}
