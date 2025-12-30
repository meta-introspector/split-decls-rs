// Generated macro for impl_2068 (impl)
macro_rules! Depcrate_sql_types_opsimpl_2068 {
() => {
// Module: crate::sql_types::ops
// Provides: {"impl_2068"}
// Dependencies: {}
impl < T > Div for Nullable < T > where T : Div + SqlType < IsNull = is_nullable :: NotNull > , T :: Rhs : SqlType < IsNull = is_nullable :: NotNull > , T :: Output : SqlType < IsNull = is_nullable :: NotNull > , { type Rhs = Nullable < T :: Rhs > ; type Output = Nullable < T :: Output > ; }
};
}
