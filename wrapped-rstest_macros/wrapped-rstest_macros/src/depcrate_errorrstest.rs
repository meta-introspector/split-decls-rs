// Generated macro for rstest (function)
macro_rules! Depcrate_errorrstest {
() => {
// Module: crate::error
// Provides: {"rstest"}
// Dependencies: {}
pub (crate) fn rstest (test : & ItemFn , info : & RsTestInfo) -> TokenStream { missed_arguments (test , info . data . items . iter ()) . chain (duplicate_arguments (info . data . items . iter ())) . chain (invalid_cases (& info . data)) . chain (case_args_without_cases (& info . data)) . chain (destruct_fixture_without_from (test , info)) . chain (test_attributes (test , & info . arguments)) . map (| e | e . to_compile_error ()) . collect () }
};
}
