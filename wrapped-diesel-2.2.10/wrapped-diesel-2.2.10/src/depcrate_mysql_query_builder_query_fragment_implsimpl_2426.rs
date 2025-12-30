// Generated macro for impl_2426 (impl)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsimpl_2426 {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"impl_2426"}
// Dependencies: {}
impl < C > DoNothingClauseHelper for C where C : Column , { fn walk_ast < T > (mut out : AstPass < '_ , '_ , Mysql >) -> QueryResult < () > where T : StaticQueryFragment , T :: Component : QueryFragment < Mysql > , { T :: STATIC_COMPONENT . walk_ast (out . reborrow ()) ? ; out . push_sql (".") ; out . push_identifier (C :: NAME) ? ; out . push_sql (" = ") ; T :: STATIC_COMPONENT . walk_ast (out . reborrow ()) ? ; out . push_sql (".") ; out . push_identifier (C :: NAME) ? ; Ok (()) } }
};
}
