// Generated macro for impl_2043 (impl)
macro_rules! Depcrate_sql_types_foldimpl_2043 {
() => {
// Module: crate::sql_types::fold
// Provides: {"impl_2043"}
// Dependencies: {}
impl < T > Foldable for sql_types :: Nullable < T > where T : Foldable + SqlType < IsNull = is_nullable :: NotNull > , { type Sum = T :: Sum ; type Avg = T :: Avg ; }
};
}
