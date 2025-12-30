// Generated macro for test (module)
macro_rules! Depcrate_pg_query_builder_tablesampletest {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: backend :: Backend ; use crate :: query_builder :: QueryBuilder ; use diesel :: dsl :: * ; use diesel :: * ; macro_rules ! assert_sql { ($ query : expr , $ sql : expr) => { let mut query_builder = < Pg as Backend >:: QueryBuilder :: default () ; $ query . to_sql (& mut query_builder , & Pg) . unwrap () ; let sql = query_builder . finish () ; assert_eq ! (sql , $ sql) ; } ; } table ! { users { id -> Integer , name -> VarChar , } } # [test] fn test_generated_tablesample_sql () { assert_sql ! (users :: table . tablesample_bernoulli (10) , "\"users\" TABLESAMPLE BERNOULLI($1)") ; assert_sql ! (users :: table . tablesample_system (10) , "\"users\" TABLESAMPLE SYSTEM($1)") ; assert_sql ! (users :: table . tablesample_system (10) . with_seed (42.0) , "\"users\" TABLESAMPLE SYSTEM($1) REPEATABLE($2)") ; } }
};
}
