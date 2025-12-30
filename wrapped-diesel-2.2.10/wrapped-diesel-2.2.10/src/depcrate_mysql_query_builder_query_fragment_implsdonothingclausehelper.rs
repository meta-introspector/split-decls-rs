// Generated macro for DoNothingClauseHelper (trait)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsDoNothingClauseHelper {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"DoNothingClauseHelper"}
// Dependencies: {}
# [doc = " This is a helper trait"] # [doc = " that provideds a fake `DO NOTHING` clause"] # [doc = " based on reassigning the possible"] # [doc = " composite primary key to itself"] trait DoNothingClauseHelper { fn walk_ast < T > (out : AstPass < '_ , '_ , Mysql >) -> QueryResult < () > where T : StaticQueryFragment , T :: Component : QueryFragment < Mysql > ; }
};
}
