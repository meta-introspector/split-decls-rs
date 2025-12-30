// Generated macro for impl_2065 (impl)
macro_rules! Depcrate_sql_types_opsimpl_2065 {
() => {
// Module: crate::sql_types::ops
// Provides: {"impl_2065"}
// Dependencies: {}
impl < T > Add for Nullable < T > where T : Add + SqlType < IsNull = is_nullable :: NotNull > , T :: Rhs : SqlType < IsNull = is_nullable :: NotNull > , T :: Output : SqlType < IsNull = is_nullable :: NotNull > , { type Rhs = Nullable < T :: Rhs > ; type Output = Nullable < T :: Output > ; }
};
}
