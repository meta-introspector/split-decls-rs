// Generated macro for impl_1180 (impl)
macro_rules! Depcrate_query_builder_select_statement_boxedimpl_1180 {
() => {
// Module: crate::query_builder::select_statement::boxed
// Provides: {"impl_1180"}
// Dependencies: {}
impl < ST , QS , DB , Rhs > JoinTo < Rhs > for BoxedSelectStatement < '_ , ST , FromClause < QS > , DB , () > where QS : JoinTo < Rhs > + QuerySource , { type FromClause = < QS as JoinTo < Rhs > > :: FromClause ; type OnClause = QS :: OnClause ; fn join_target (rhs : Rhs) -> (Self :: FromClause , Self :: OnClause) { QS :: join_target (rhs) } }
};
}
