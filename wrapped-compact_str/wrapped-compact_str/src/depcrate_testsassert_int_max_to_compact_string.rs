// Generated macro for assert_int_MAX_to_compact_string (macro)
macro_rules! Depcrate_testsassert_int_MAX_to_compact_string {
() => {
// Module: crate::tests
// Provides: {"assert_int_MAX_to_compact_string"}
// Dependencies: {}
macro_rules ! assert_int_MAX_to_compact_string { ($ int : ty) => { assert_eq ! (&*<$ int >:: MAX . to_string () , &*<$ int >:: MAX . to_compact_string ()) ; } ; }
};
}
