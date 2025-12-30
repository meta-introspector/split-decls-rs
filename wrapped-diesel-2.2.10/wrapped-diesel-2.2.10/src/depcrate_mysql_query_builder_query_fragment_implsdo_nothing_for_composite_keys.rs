// Generated macro for do_nothing_for_composite_keys (macro)
macro_rules! Depcrate_mysql_query_builder_query_fragment_implsdo_nothing_for_composite_keys {
() => {
// Module: crate::mysql::query_builder::query_fragment_impls
// Provides: {"do_nothing_for_composite_keys"}
// Dependencies: {}
macro_rules ! do_nothing_for_composite_keys { ($ ($ Tuple : tt { $ (($ idx : tt) -> $ T : ident , $ ST : ident , $ TT : ident ,) + }) +) => { $ (impl <$ ($ T ,) *> DoNothingClauseHelper for ($ ($ T ,) *) where $ ($ T : Column ,) * { fn walk_ast < Table > (mut out : AstPass <'_ , '_ , Mysql >) -> QueryResult < () > where Table : StaticQueryFragment , Table :: Component : QueryFragment < Mysql >, { let mut first = true ; $ (# [allow (unused_assignments)] if first { first = false ; } else { out . push_sql (", ") ; } Table :: STATIC_COMPONENT . walk_ast (out . reborrow ()) ?; out . push_sql (".") ; out . push_identifier ($ T :: NAME) ?; out . push_sql (" = ") ; Table :: STATIC_COMPONENT . walk_ast (out . reborrow ()) ?; out . push_sql (".") ; out . push_identifier ($ T :: NAME) ?;) * Ok (()) } }) * } }
};
}
