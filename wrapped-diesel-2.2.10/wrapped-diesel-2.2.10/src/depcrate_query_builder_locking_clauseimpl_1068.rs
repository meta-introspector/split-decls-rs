// Generated macro for impl_1068 (impl)
macro_rules! Depcrate_query_builder_locking_clauseimpl_1068 {
() => {
// Module: crate::query_builder::locking_clause
// Provides: {"impl_1068"}
// Dependencies: {}
impl < DB , L , M > QueryFragment < DB > for LockingClause < L , M > where DB : Backend + DieselReserveSpecialization , L : QueryFragment < DB > , M : QueryFragment < DB > , { fn walk_ast < 'b > (& 'b self , mut out : AstPass < '_ , 'b , DB >) -> QueryResult < () > { self . lock_mode . walk_ast (out . reborrow ()) ? ; self . modifier . walk_ast (out . reborrow ()) } }
};
}
