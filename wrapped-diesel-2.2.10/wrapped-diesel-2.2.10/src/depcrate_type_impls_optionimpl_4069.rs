// Generated macro for impl_4069 (impl)
macro_rules! Depcrate_type_impls_optionimpl_4069 {
() => {
// Module: crate::type_impls::option
// Provides: {"impl_4069"}
// Dependencies: {}
impl < T , DB > HasSqlType < Nullable < T > > for DB where DB : Backend + HasSqlType < T > , T : SqlType , { fn metadata (lookup : & mut DB :: MetadataLookup) -> DB :: TypeMetadata { < DB as HasSqlType < T > > :: metadata (lookup) } }
};
}
