// Generated macro for impl_4019 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4019 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4019"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `Vec<u8>`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "sqlite")] impl FromSql < sql_types :: Binary , Sqlite > for * const [u8] { fn from_sql (mut bytes : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { let bytes = bytes . read_blob () ; Ok (bytes as * const _) } }
};
}
