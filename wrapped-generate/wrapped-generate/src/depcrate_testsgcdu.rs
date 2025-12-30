// Generated macro for gcdu (function)
macro_rules! Depcrate_testsgcdu {
() => {
// Module: crate::tests
// Provides: {"gcdu"}
// Dependencies: {}
fn gcdu (mut a : u64 , mut b : u64) -> u64 { while a != 0 { let tmp = b % a ; b = a ; a = tmp ; } b }
};
}
