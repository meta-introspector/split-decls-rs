// Generated macro for impl_3922 (impl)
macro_rules! Depcrate_sqlite_query_builderimpl_3922 {
() => {
// Module: crate::sqlite::query_builder
// Provides: {"impl_3922"}
// Dependencies: {}
impl QueryBuilder < Sqlite > for SqliteQueryBuilder { fn push_sql (& mut self , sql : & str) { self . sql . push_str (sql) ; } fn push_identifier (& mut self , identifier : & str) -> QueryResult < () > { self . push_sql ("`") ; self . push_sql (& identifier . replace ('`' , "``")) ; self . push_sql ("`") ; Ok (()) } fn push_bind_param (& mut self) { self . push_sql ("?") ; } fn finish (self) -> String { self . sql } }
};
}
