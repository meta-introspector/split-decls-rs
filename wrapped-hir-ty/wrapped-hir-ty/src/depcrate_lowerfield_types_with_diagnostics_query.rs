// Generated macro for field_types_with_diagnostics_query (function)
macro_rules! Depcrate_lowerfield_types_with_diagnostics_query {
() => {
// Module: crate::lower
// Provides: {"field_types_with_diagnostics_query"}
// Dependencies: {}
# [doc = " Build the type of all specific fields of a struct or enum variant."] pub (crate) fn field_types_with_diagnostics_query < 'db > (db : & 'db dyn HirDatabase , variant_id : VariantId ,) -> (Arc < ArenaMap < LocalFieldId , EarlyBinder < 'db , Ty < 'db > > > > , Diagnostics) { let var_data = variant_id . fields (db) ; let fields = var_data . fields () ; if fields . is_empty () { return (Arc :: new (ArenaMap :: default ()) , None) ; } let (resolver , def) : (_ , GenericDefId) = match variant_id { VariantId :: StructId (it) => (it . resolver (db) , it . into ()) , VariantId :: UnionId (it) => (it . resolver (db) , it . into ()) , VariantId :: EnumVariantId (it) => (it . resolver (db) , it . lookup (db) . parent . into ()) , } ; let mut res = ArenaMap :: default () ; let mut ctx = TyLoweringContext :: new (db , & resolver , & var_data . store , def , LifetimeElisionKind :: AnonymousReportError ,) ; for (field_id , field_data) in var_data . fields () . iter () { res . insert (field_id , EarlyBinder :: bind (ctx . lower_ty (field_data . type_ref))) ; } (Arc :: new (res) , create_diagnostics (ctx . diagnostics)) }
};
}
