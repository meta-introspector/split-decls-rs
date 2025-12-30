// Generated macro for prepend (function)
macro_rules! Depcrate_tests_ownedmodulusvalue_testsprepend {
() => {
// Module: crate::tests::ownedmodulusvalue_tests
// Provides: {"prepend"}
// Dependencies: {}
fn prepend (a : u8 , b : & [u8]) -> Vec < u8 > { let mut r = Vec :: with_capacity (b . len () + 1) ; r . push (a) ; r . extend_from_slice (b) ; r }
};
}
