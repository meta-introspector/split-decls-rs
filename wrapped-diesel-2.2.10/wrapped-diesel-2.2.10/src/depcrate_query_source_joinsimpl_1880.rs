// Generated macro for impl_1880 (impl)
macro_rules! Depcrate_query_source_joinsimpl_1880 {
() => {
// Module: crate::query_source::joins
// Provides: {"impl_1880"}
// Dependencies: {}
impl < Join , On , Right > JoinTo < Right > for JoinOn < Join , On > where Join : JoinTo < Right > , { type FromClause = Join :: FromClause ; type OnClause = Join :: OnClause ; fn join_target (rhs : Right) -> (Self :: FromClause , Self :: OnClause) { Join :: join_target (rhs) } }
};
}
