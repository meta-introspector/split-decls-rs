// Generated macro for handle_as_ref_str (function)
macro_rules! Depcrate_utilshandle_as_ref_str {
() => {
// Module: crate::utils
// Provides: {"handle_as_ref_str"}
// Dependencies: {}
fn handle_as_ref_str (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { let str_type = hir :: BuiltinType :: str () . ty (db) ; ty . impls_trait (db , famous_defs . core_convert_AsRef () ? , slice :: from_ref (& str_type)) . then_some ((ReferenceConversionType :: AsRefStr , could_deref_to_target (ty , & str_type , db))) }
};
}
