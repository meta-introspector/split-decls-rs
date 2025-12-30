// Generated macro for impl_2137 (impl)
macro_rules! Depcrate_sql_typesimpl_2137 {
() => {
// Module: crate::sql_types
// Provides: {"impl_2137"}
// Dependencies: {}
impl < T > IntoNullable for T where T : SqlType < IsNull = is_nullable :: NotNull > + SingleValue , { type Nullable = Nullable < T > ; }
};
}
