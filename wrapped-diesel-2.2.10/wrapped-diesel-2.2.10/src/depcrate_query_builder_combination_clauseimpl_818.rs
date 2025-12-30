// Generated macro for impl_818 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_818 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_818"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for Intersect where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" INTERSECT ") ; Ok (()) } }
};
}
