// Generated macro for impl_608 (impl)
macro_rules! Depcrate_expressionimpl_608 {
() => {
// Module: crate::expression
// Provides: {"impl_608"}
// Dependencies: {}
impl < T , DB > QueryMetadata < T > for DB where DB : Backend + HasSqlType < T > , T : SingleValue , { fn row_metadata (lookup : & mut Self :: MetadataLookup , out : & mut Vec < Option < Self :: TypeMetadata > >) { out . push (Some (< DB as HasSqlType < T > > :: metadata (lookup))) } }
};
}
