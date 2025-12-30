// Generated macro for impl_1829 (impl)
macro_rules! Depcrate_query_source_aliasing_joinsimpl_1829 {
() => {
// Module: crate::query_source::aliasing::joins
// Provides: {"impl_1829"}
// Dependencies: {}
impl < S , Rhs , On > JoinTo < OnClauseWrapper < Rhs , On > > for Alias < S > { type FromClause = Rhs ; type OnClause = On ; fn join_target (rhs : OnClauseWrapper < Rhs , On >) -> (Self :: FromClause , Self :: OnClause) { (rhs . source , rhs . on) } }
};
}
