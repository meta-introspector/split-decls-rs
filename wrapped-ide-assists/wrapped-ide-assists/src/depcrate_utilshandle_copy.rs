// Generated macro for handle_copy (function)
macro_rules! Depcrate_utilshandle_copy {
() => {
// Module: crate::utils
// Provides: {"handle_copy"}
// Dependencies: {}
fn handle_copy (ty : & hir :: Type < '_ > , db : & dyn HirDatabase ,) -> Option < (ReferenceConversionType , bool) > { ty . is_copy (db) . then_some ((ReferenceConversionType :: Copy , true)) }
};
}
