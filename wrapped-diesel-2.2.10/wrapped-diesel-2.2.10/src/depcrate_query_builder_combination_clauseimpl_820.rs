// Generated macro for impl_820 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_820 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_820"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for Except where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" EXCEPT ") ; Ok (()) } }
};
}
