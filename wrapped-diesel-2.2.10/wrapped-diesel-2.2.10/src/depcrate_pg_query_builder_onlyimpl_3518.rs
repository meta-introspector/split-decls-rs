// Generated macro for impl_3518 (impl)
macro_rules! Depcrate_pg_query_builder_onlyimpl_3518 {
() => {
// Module: crate::pg::query_builder::only
// Provides: {"impl_3518"}
// Dependencies: {}
impl < S , T > JoinTo < T > for Only < S > where S : JoinTo < T > , T : Table , S : Table , { type FromClause = < S as JoinTo < T > > :: FromClause ; type OnClause = < S as JoinTo < T > > :: OnClause ; fn join_target (rhs : T) -> (Self :: FromClause , Self :: OnClause) { < S as JoinTo < T > > :: join_target (rhs) } }
};
}
