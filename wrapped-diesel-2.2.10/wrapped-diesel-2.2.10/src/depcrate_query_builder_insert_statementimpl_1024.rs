// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_query_builder_insert_statementimpl_1024 {
() => {
// Module: crate::query_builder::insert_statement
// Provides: {"impl_1024"}
// Dependencies: {}
impl < T , U , Op , Ret , DB > QueryFragment < DB > for InsertStatement < T , U , Op , Ret > where DB : Backend + DieselReserveSpecialization , T : Table , T :: FromClause : QueryFragment < DB > , U : QueryFragment < DB > + CanInsertInSingleQuery < DB > , Op : QueryFragment < DB > , Ret : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { if self . records . rows_to_insert () == Some (0) { out . push_sql ("SELECT 1 FROM ") ; self . into_clause . walk_ast (out . reborrow ()) ? ; out . push_sql (" WHERE 1=0") ; return Ok (()) ; } self . operator . walk_ast (out . reborrow ()) ? ; out . push_sql (" INTO ") ; self . into_clause . walk_ast (out . reborrow ()) ? ; out . push_sql (" ") ; self . records . walk_ast (out . reborrow ()) ? ; self . returning . walk_ast (out . reborrow ()) ? ; Ok (()) } }
};
}
