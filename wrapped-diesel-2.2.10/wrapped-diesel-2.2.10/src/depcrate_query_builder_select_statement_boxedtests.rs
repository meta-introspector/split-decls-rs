// Generated macro for tests (module)
macro_rules! Depcrate_query_builder_select_statement_boxedtests {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: prelude :: * ; table ! { users { id -> Integer , } } fn assert_send < T > (_ : T) where T : Send , { } macro_rules ! assert_boxed_query_send { ($ backend : ty) => { { assert_send (users :: table . into_boxed ::<$ backend > ()) ; assert_send (users :: table . filter (users :: id . eq (10)) . into_boxed ::<$ backend > () ,) ; } ; } ; } # [test] fn boxed_is_send () { # [cfg (feature = "postgres")] assert_boxed_query_send ! (crate :: pg :: Pg) ; # [cfg (feature = "sqlite")] assert_boxed_query_send ! (crate :: sqlite :: Sqlite) ; # [cfg (feature = "mysql")] assert_boxed_query_send ! (crate :: mysql :: Mysql) ; } }
};
}
