// Generated macro for impl_2067 (impl)
macro_rules! Depcrate_sql_types_opsimpl_2067 {
() => {
// Module: crate::sql_types::ops
// Provides: {"impl_2067"}
// Dependencies: {}
impl < T > Mul for Nullable < T > where T : Mul + SqlType < IsNull = is_nullable :: NotNull > , T :: Rhs : SqlType < IsNull = is_nullable :: NotNull > , T :: Output : SqlType < IsNull = is_nullable :: NotNull > , { type Rhs = Nullable < T :: Rhs > ; type Output = Nullable < T :: Output > ; }
};
}
