// Generated macro for append (function)
macro_rules! Depcrate_tests_ownedmodulusvalue_testsappend {
() => {
// Module: crate::tests::ownedmodulusvalue_tests
// Provides: {"append"}
// Dependencies: {}
fn append (a : & [u8] , b : u8) -> Vec < u8 > { let mut r = Vec :: with_capacity (a . len () + 1) ; r . extend_from_slice (a) ; r . push (b) ; r }
};
}
