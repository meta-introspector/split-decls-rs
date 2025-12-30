// Generated macro for fields_attrs_source_map (function)
macro_rules! Depcrate_attrfields_attrs_source_map {
() => {
// Module: crate::attr
// Provides: {"fields_attrs_source_map"}
// Dependencies: {}
pub (crate) fn fields_attrs_source_map (db : & dyn DefDatabase , def : VariantId ,) -> Arc < ArenaMap < LocalFieldId , AstPtr < Either < ast :: TupleField , ast :: RecordField > > > > { let mut res = ArenaMap :: default () ; let child_source = def . child_source (db) ; for (idx , variant) in child_source . value . iter () { res . insert (idx , variant . as_ref () . either (| l | AstPtr :: new (l) . wrap_left () , | r | AstPtr :: new (r) . wrap_right ()) ,) ; } Arc :: new (res) }
};
}
