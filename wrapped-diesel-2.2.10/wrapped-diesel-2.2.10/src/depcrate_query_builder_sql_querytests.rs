// Generated macro for tests (module)
macro_rules! Depcrate_query_builder_sql_querytests {
() => {
// Module: crate::query_builder::sql_query
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { fn assert_send < S : Send > (_ : S) { } # [test] fn check_boxed_sql_query_is_send () { let query = crate :: sql_query ("SELECT 1") . into_boxed :: < < crate :: test_helpers :: TestConnection as crate :: Connection > :: Backend > () ; assert_send (query) ; } }
};
}
