// Generated macro for check_sql_query_increments_bind_count (function)
macro_rules! Depcrate_pg_query_buildercheck_sql_query_increments_bind_count {
() => {
// Module: crate::pg::query_builder
// Provides: {"check_sql_query_increments_bind_count"}
// Dependencies: {}
# [test] fn check_sql_query_increments_bind_count () { use crate :: query_builder :: { AstPass , AstPassToSqlOptions , QueryFragment } ; use crate :: sql_types :: * ; let query = crate :: sql_query ("SELECT $1, $2, $3") . bind :: < Integer , _ > (42) . bind :: < Integer , _ > (3) . bind :: < Integer , _ > (342) ; let mut query_builder = PgQueryBuilder :: default () ; { let mut options = AstPassToSqlOptions :: default () ; let ast_pass = AstPass :: < crate :: pg :: Pg > :: to_sql (& mut query_builder , & mut options , & Pg) ; query . walk_ast (ast_pass) . unwrap () ; } assert_eq ! (query_builder . bind_idx , 3) ; assert_eq ! (query_builder . sql , "SELECT $1, $2, $3") ; }
};
}
