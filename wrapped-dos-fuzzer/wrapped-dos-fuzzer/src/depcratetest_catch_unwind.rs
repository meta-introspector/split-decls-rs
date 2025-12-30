// Generated macro for test_catch_unwind (function)
macro_rules! Depcratetest_catch_unwind {
() => {
// Module: crate
// Provides: {"test_catch_unwind"}
// Dependencies: {}
fn test_catch_unwind (pattern : & Pattern) -> Result < PatternResult , () > { let res = panic :: catch_unwind (| | test (& pattern)) ; if res . is_err () { println ! ("Panic caught. Pattern: {}" , serde_json :: to_string (& pattern) . unwrap ()) ; } res . map_err (| _ | ()) }
};
}
