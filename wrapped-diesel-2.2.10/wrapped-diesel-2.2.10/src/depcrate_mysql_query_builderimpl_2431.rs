// Generated macro for impl_2431 (impl)
macro_rules! Depcrate_mysql_query_builderimpl_2431 {
() => {
// Module: crate::mysql::query_builder
// Provides: {"impl_2431"}
// Dependencies: {}
impl QueryBuilder < Mysql > for MysqlQueryBuilder { fn push_sql (& mut self , sql : & str) { self . sql . push_str (sql) ; } fn push_identifier (& mut self , identifier : & str) -> QueryResult < () > { self . push_sql ("`") ; self . push_sql (& identifier . replace ('`' , "``")) ; self . push_sql ("`") ; Ok (()) } fn push_bind_param (& mut self) { self . push_sql ("?") ; } fn finish (self) -> String { self . sql } }
};
}
