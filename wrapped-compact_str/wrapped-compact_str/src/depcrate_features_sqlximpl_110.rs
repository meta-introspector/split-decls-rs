// Generated macro for impl_110 (impl)
macro_rules! Depcrate_features_sqlximpl_110 {
() => {
// Module: crate::features::sqlx
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "sqlx-postgres")] # [cfg_attr (docsrs , doc (cfg (feature = "sqlx-postgres")))] impl sqlx :: postgres :: PgHasArrayType for CompactString { fn array_type_info () -> sqlx :: postgres :: PgTypeInfo { < std :: string :: String as sqlx :: postgres :: PgHasArrayType > :: array_type_info () } }
};
}
