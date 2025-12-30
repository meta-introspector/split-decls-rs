// Generated macro for list (function)
macro_rules! Depcrate_tests_utilslist {
() => {
// Module: crate::tests::utils
// Provides: {"list"}
// Dependencies: {}
pub (crate) fn list () -> Vec < String > { let dylib_path = proc_macro_test_dylib_path () ; let env = EnvSnapshot :: default () ; let srv = ProcMacroSrv :: new (& env) ; let res = srv . list_macros (& dylib_path) . unwrap () ; res . into_iter () . map (| (name , kind) | format ! ("{name} [{kind:?}]")) . collect () }
};
}
