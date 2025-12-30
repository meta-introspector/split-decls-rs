// Generated macro for impl_2140 (impl)
macro_rules! Depcrate_sql_typesimpl_2140 {
() => {
// Module: crate::sql_types
// Provides: {"impl_2140"}
// Dependencies: {}
impl < T > IntoNotNullable for T where T : SqlType < IsNull = is_nullable :: NotNull > , { type NotNullable = T ; }
};
}
