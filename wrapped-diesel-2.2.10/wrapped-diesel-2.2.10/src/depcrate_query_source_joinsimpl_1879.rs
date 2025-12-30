// Generated macro for impl_1879 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1879 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1879"}
// Dependencies: {}
impl < Left , Mid , Right , Kind > JoinTo < Right > for Join < Left , Mid , Kind > where Left : JoinTo < Right > + QuerySource , Mid : QuerySource , { type FromClause = < Left as JoinTo < Right > > :: FromClause ; type OnClause = Left :: OnClause ; fn join_target (rhs : Right) -> (Self :: FromClause , Self :: OnClause) { Left :: join_target (rhs) } }
};
}
