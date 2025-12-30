// Generated macro for handle_result_as_ref (function)
macro_rules! Depcrate_utilshandle_result_as_ref {
() => {
// Module: crate::utils
// Provides: {"handle_result_as_ref"}
// Dependencies: {}
fn handle_result_as_ref (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { if ty . as_adt () == famous_defs . core_result_Result () ? . ty (db) . as_adt () { Some ((ReferenceConversionType :: Result , false)) } else { None } }
};
}
