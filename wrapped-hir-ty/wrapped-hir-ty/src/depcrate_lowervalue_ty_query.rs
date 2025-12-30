// Generated macro for value_ty_query (function)
macro_rules! Depcrate_lowervalue_ty_query {
() => {
// Module: crate::lower
// Provides: {"value_ty_query"}
// Dependencies: {}
pub (crate) fn value_ty_query < 'db > (db : & 'db dyn HirDatabase , def : ValueTyDefId ,) -> Option < EarlyBinder < 'db , Ty < 'db > > > { match def { ValueTyDefId :: FunctionId (it) => Some (type_for_fn (db , it)) , ValueTyDefId :: StructId (it) => type_for_struct_constructor (db , it) , ValueTyDefId :: UnionId (it) => Some (type_for_adt (db , it . into ())) , ValueTyDefId :: EnumVariantId (it) => type_for_enum_variant_constructor (db , it) , ValueTyDefId :: ConstId (it) => Some (type_for_const (db , it)) , ValueTyDefId :: StaticId (it) => Some (type_for_static (db , it)) , } }
};
}
