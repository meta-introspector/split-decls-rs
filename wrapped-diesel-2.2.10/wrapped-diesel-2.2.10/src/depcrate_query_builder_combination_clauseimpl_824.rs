// Generated macro for impl_824 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_824 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_824"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for All where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql ("ALL ") ; Ok (()) } }
};
}
