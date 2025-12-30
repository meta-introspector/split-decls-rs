// Generated macro for impl_264 (impl)
macro_rules! Depcrate_lowerimpl_264 {
() => {
// Module: crate::lower
// Provides: {"impl_264"}
// Dependencies: {}
impl ValueTyDefId { pub (crate) fn to_generic_def_id (self , db : & dyn HirDatabase) -> GenericDefId { match self { Self :: FunctionId (id) => id . into () , Self :: StructId (id) => id . into () , Self :: UnionId (id) => id . into () , Self :: EnumVariantId (var) => var . lookup (db) . parent . into () , Self :: ConstId (id) => id . into () , Self :: StaticId (id) => id . into () , } } }
};
}
