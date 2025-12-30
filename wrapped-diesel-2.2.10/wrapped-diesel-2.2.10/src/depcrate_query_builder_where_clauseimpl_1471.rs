// Generated macro for impl_1471 (impl)
macro_rules! Depcrate_query_builder_where_clauseimpl_1471 {
() => {
// Module: crate::query_builder::where_clause
// Provides: {"impl_1471"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoWhereClause where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
