// Generated macro for entry_point_type (function)
macro_rules! Depcrate_test_harnessentry_point_type {
() => {
// Module: crate::test_harness
// Provides: {"entry_point_type"}
// Dependencies: {}
fn entry_point_type (item : & ast :: Item , at_root : bool) -> EntryPointType { match & item . kind { ast :: ItemKind :: Fn (fn_) => { rustc_ast :: entry :: entry_point_type (& item . attrs , at_root , Some (fn_ . ident . name)) } _ => EntryPointType :: None , } }
};
}
