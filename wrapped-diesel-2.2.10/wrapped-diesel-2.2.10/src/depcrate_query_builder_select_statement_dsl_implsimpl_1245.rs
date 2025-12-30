// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_query_builder_select_statement_dsl_implsimpl_1245 {
() => {
// Module: crate::query_builder::select_statement::dsl_impls
// Provides: {"impl_1245"}
// Dependencies: {}
impl < F , S , D , W , O , LOf , G , H , LC , Rhs > JoinTo < Rhs > for SelectStatement < FromClause < F > , S , D , W , O , LOf , G , H , LC > where F : JoinTo < Rhs > + QuerySource , { type FromClause = < F as JoinTo < Rhs > > :: FromClause ; type OnClause = F :: OnClause ; fn join_target (rhs : Rhs) -> (Self :: FromClause , Self :: OnClause) { F :: join_target (rhs) } }
};
}
