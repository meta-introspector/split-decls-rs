// Generated macro for impl_1485 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1485 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1485"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for BoxedWhereClause < '_ , DB > where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { match * self { BoxedWhereClause :: Where (ref where_clause) => { out . push_sql (" WHERE ") ; where_clause . walk_ast (out) } BoxedWhereClause :: None => Ok (()) , } } }
};
}
