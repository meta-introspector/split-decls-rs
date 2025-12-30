// Generated macro for impl_816 (impl)
macro_rules! Depcrate_query_builder_combination_clauseimpl_816 {
() => {
// Module: crate::query_builder::combination_clause
// Provides: {"impl_816"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for Union where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { out . push_sql (" UNION ") ; Ok (()) } }
};
}
