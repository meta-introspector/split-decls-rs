// Generated macro for impl_4020 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4020 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4020"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl Queryable < sql_types :: Binary , Sqlite > for * const [u8] { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
