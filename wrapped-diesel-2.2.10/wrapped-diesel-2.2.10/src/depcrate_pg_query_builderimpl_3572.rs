// Generated macro for impl_3572 (impl)
macro_rules! Depcrate_pg_query_builderimpl_3572 {
() => {
// Module: crate::pg::query_builder
// Provides: {"impl_3572"}
// Dependencies: {}
impl QueryBuilder < Pg > for PgQueryBuilder { fn push_sql (& mut self , sql : & str) { self . sql . push_str (sql) ; } fn push_identifier (& mut self , identifier : & str) -> QueryResult < () > { self . push_sql ("\"") ; self . push_sql (& identifier . replace ('"' , "\"\"")) ; self . push_sql ("\"") ; Ok (()) } fn push_bind_param (& mut self) { self . push_bind_param_value_only () ; self . sql += "$" ; let mut buffer = itoa :: Buffer :: new () ; self . sql += buffer . format (self . bind_idx) ; } fn push_bind_param_value_only (& mut self) { self . bind_idx += 1 ; } fn finish (self) -> String { self . sql } }
};
}
