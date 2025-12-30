// Generated macro for impl_2082 (impl)
macro_rules! Depcrate_sql_types_ordimpl_2082 {
() => {
// Module: crate::sql_types::ord
// Provides: {"impl_2082"}
// Dependencies: {}
impl < T > SqlOrd for sql_types :: Nullable < T > where T : SqlOrd + SqlType < IsNull = is_nullable :: NotNull > { }
};
}
