// Generated macro for impl_1107 (impl)
macro_rules! Depcrate_query_builder_returning_clauseimpl_1107 {
() => {
// Module: crate::query_builder::returning_clause
// Provides: {"impl_1107"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoReturningClause where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
