// Generated macro for impl_1065 (impl)
macro_rules! Depcrate_query_builder_locking_clauseimpl_1065 {
() => {
// Module: crate::query_builder::locking_clause
// Provides: {"impl_1065"}
// Dependencies: {}
impl < DB > QueryFragment < DB > for NoLockingClause where DB : Backend + DieselReserveSpecialization , { fn walk_ast < 'b > (& 'b self , _ : AstPass < '_ , 'b , DB >) -> QueryResult < () > { Ok (()) } }
};
}
