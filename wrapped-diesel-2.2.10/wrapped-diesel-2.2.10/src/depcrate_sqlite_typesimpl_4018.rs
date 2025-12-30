// Generated macro for impl_4018 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4018 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4018"}
// Dependencies: {}
# [cfg (feature = "sqlite")] impl Queryable < sql_types :: VarChar , Sqlite > for * const str { type Row = Self ; fn build (row : Self :: Row) -> deserialize :: Result < Self > { Ok (row) } }
};
}
