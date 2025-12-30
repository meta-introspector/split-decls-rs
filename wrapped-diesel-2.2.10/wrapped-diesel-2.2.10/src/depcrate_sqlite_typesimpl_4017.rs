// Generated macro for impl_4017 (impl)
macro_rules! Depcrate_sqlite_typesimpl_4017 {
() => {
// Module: crate::sqlite::types
// Provides: {"impl_4017"}
// Dependencies: {}
# [doc = " The returned pointer is *only* valid for the lifetime to the argument of"] # [doc = " `from_sql`. This impl is intended for uses where you want to write a new"] # [doc = " impl in terms of `String`, but don't want to allocate. We have to return a"] # [doc = " raw pointer instead of a reference with a lifetime due to the structure of"] # [doc = " `FromSql`"] # [cfg (feature = "sqlite")] impl FromSql < sql_types :: VarChar , Sqlite > for * const str { fn from_sql (mut value : SqliteValue < '_ , '_ , '_ >) -> deserialize :: Result < Self > { let text = value . read_text () ; Ok (text as * const _) } }
};
}
