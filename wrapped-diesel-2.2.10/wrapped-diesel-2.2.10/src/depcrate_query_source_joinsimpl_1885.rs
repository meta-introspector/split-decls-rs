// Generated macro for impl_1885 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1885 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1885"}
// Dependencies: {}
impl < Lhs , Rhs , On > JoinTo < OnClauseWrapper < Rhs , On > > for Lhs where Lhs : Table , { type FromClause = Rhs ; type OnClause = On ; fn join_target (rhs : OnClauseWrapper < Rhs , On >) -> (Self :: FromClause , Self :: OnClause) { (rhs . source , rhs . on) } }
};
}
