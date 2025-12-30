// Generated macro for impl_877 (impl)
macro_rules! Depcrate_query_builder_distinct_clauseimpl_877 {
() => {
// Module: crate::query_builder::distinct_clause
// Provides: {"impl_877"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for DistinctClause where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("DISTINCT ") ; Ok (()) } }
};
}
