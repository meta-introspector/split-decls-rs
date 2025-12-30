// Generated macro for impl_3131 (impl)
macro_rules! Depcrate_pg_types_recordimpl_3131 {
() => {
// Module: crate::pg::types::record
// Provides: {"impl_3131"}
// Dependencies: {}
impl < T > Expression for PgTuple < T > where T : Expression , T :: SqlType : 'static , { type SqlType = Record < T :: SqlType > ; }
};
}
