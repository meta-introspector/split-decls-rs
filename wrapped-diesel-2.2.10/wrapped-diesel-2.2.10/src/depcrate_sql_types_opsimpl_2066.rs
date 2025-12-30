// Generated macro for impl_2066 (impl)
macro_rules! Depcrate_sql_types_opsimpl_2066 {
() => {
// Module: crate::sql_types::ops
// Provides: {"impl_2066"}
// Dependencies: {}
impl < T > Sub for Nullable < T > where T : Sub + SqlType < IsNull = is_nullable :: NotNull > , T :: Rhs : SqlType < IsNull = is_nullable :: NotNull > , T :: Output : SqlType < IsNull = is_nullable :: NotNull > , { type Rhs = Nullable < T :: Rhs > ; type Output = Nullable < T :: Output > ; }
};
}
