// Generated macro for impl_3269 (impl)
macro_rules! Depcrate_pg_connection_rowimpl_3269 {
() => {
// Module: crate::pg::connection::row
// Provides: {"impl_3269"}
// Dependencies: {}
impl TypeOidLookup for PgField < '_ > { fn lookup (& self) -> std :: num :: NonZeroU32 { self . db_result . column_type (self . col_idx) } }
};
}
