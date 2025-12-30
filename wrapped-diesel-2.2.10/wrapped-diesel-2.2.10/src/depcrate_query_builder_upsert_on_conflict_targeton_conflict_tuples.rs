// Generated macro for on_conflict_tuples (macro)
macro_rules! Depcrate_query_builder_upsert_on_conflict_targeton_conflict_tuples {
() => {
// Module: crate::query_builder::upsert::on_conflict_target
// Provides: {"on_conflict_tuples"}
// Dependencies: {}
macro_rules ! on_conflict_tuples { ($ ($ Tuple : tt { $ (($ idx : tt) -> $ T : ident , $ ST : ident , $ TT : ident ,) * }) +) => { $ (impl < _DB , _T , _SP , $ ($ T) ,*> QueryFragment < _DB , _SP > for ConflictTarget < (_T , $ ($ T) ,*) > where _DB : Backend < OnConflictClause = _SP >, _SP : sql_dialect :: on_conflict_clause :: PgLikeOnConflictClause , _T : Column , $ ($ T : Column < Table = _T :: Table >,) * { fn walk_ast <'b > (&'b self , mut out : AstPass <'_ , 'b , _DB >) -> QueryResult < () > { out . push_sql (" (") ; out . push_identifier (_T :: NAME) ?; $ (out . push_sql (", ") ; out . push_identifier ($ T :: NAME) ?;) * out . push_sql (")") ; Ok (()) } } impl < _T , $ ($ T) ,*> OnConflictTarget < _T :: Table > for ConflictTarget < (_T , $ ($ T) ,*) > where _T : Column , $ ($ T : Column < Table = _T :: Table >,) * { }) * } }
};
}
