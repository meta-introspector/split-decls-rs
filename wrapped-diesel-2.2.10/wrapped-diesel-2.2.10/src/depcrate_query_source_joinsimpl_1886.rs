// Generated macro for impl_1886 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1886 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1886"}
// Dependencies: {}
impl < Lhs , Rhs , On > JoinTo < Rhs > for OnClauseWrapper < Lhs , On > where Lhs : JoinTo < Rhs > , { type FromClause = < Lhs as JoinTo < Rhs > > :: FromClause ; type OnClause = < Lhs as JoinTo < Rhs > > :: OnClause ; fn join_target (rhs : Rhs) -> (Self :: FromClause , Self :: OnClause) { < Lhs as JoinTo < Rhs > > :: join_target (rhs) } }
};
}
