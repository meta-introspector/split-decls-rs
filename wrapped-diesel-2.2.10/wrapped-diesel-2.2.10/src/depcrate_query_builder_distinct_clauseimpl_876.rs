// Generated macro for impl_876 (impl)
macro_rules! Depcrate_query_builder_distinct_clauseimpl_876 {
() => {
// Module: crate::query_builder::distinct_clause
// Provides: {"impl_876"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoDistinctClause where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
