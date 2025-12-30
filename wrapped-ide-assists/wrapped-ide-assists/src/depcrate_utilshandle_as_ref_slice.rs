// Generated macro for handle_as_ref_slice (function)
macro_rules! Depcrate_utilshandle_as_ref_slice {
() => {
// Module: crate::utils
// Provides: {"handle_as_ref_slice"}
// Dependencies: {}
fn handle_as_ref_slice (ty : & hir :: Type < '_ > , db : & dyn HirDatabase , famous_defs : & FamousDefs < '_ , '_ > ,) -> Option < (ReferenceConversionType , bool) > { let type_argument = ty . type_arguments () . next () ? ; let slice_type = hir :: Type :: new_slice (type_argument) ; ty . impls_trait (db , famous_defs . core_convert_AsRef () ? , slice :: from_ref (& slice_type)) . then_some ((ReferenceConversionType :: AsRefSlice , could_deref_to_target (ty , & slice_type , db) ,)) }
};
}
