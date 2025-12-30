// Generated macro for fixture (function)
macro_rules! Depcrate_errorfixture {
() => {
// Module: crate::error
// Provides: {"fixture"}
// Dependencies: {}
pub (crate) fn fixture (test : & ItemFn , info : & FixtureInfo) -> TokenStream { missed_arguments (test , info . data . items . iter ()) . chain (duplicate_arguments (info . data . items . iter ())) . chain (async_once (test , info)) . chain (generics_once (test , info)) . chain (destruct_fixture_without_from (test , info)) . map (| e | e . to_compile_error ()) . collect () }
};
}
