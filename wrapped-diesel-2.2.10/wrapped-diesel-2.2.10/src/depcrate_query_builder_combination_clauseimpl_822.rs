// Generated macro for impl_822 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_822 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_822"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for Distinct where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
