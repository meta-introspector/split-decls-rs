// Generated macro for impl_3564 (impl)
macro_rules! Depcrate_pg_query_builder_tablesampleimpl_3564 {
() => {
// Module: crate::pg::query_builder::tablesample
// Provides: {"impl_3564"}
// Dependencies: {}
impl < S , T , TSM > JoinTo < T > for Tablesample < S , TSM > where S : JoinTo < T > , T : Table , S : Table , TSM : TablesampleMethod , { type FromClause = < S as JoinTo < T > > :: FromClause ; type OnClause = < S as JoinTo < T > > :: OnClause ; fn join_target (rhs : T) -> (Self :: FromClause , Self :: OnClause) { < S as JoinTo < T > > :: join_target (rhs) } }
};
}
