// Generated macro for size_hint_for_tuples (function)
macro_rules! Depcrate_testssize_hint_for_tuples {
() => {
// Module: crate::tests
// Provides: {"size_hint_for_tuples"}
// Dependencies: {}
# [test] fn size_hint_for_tuples () { assert_eq ! ((7 , Some (7)) , < (bool , u16 , i32) as Arbitrary <'_ >>:: size_hint (0)) ; assert_eq ! ((1 , None) , < (u8 , Vec < u8 >) as Arbitrary >:: size_hint (0)) ; }
};
}
