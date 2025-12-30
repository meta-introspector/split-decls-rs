// Generated macro for impl_1345 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1345 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1345"}
// Dependencies: {}
impl < C > AssignmentTarget for C where C : Column , { type Table = C :: Table ; type QueryAstNode = ColumnWrapperForUpdate < C > ; fn into_target (self) -> Self :: QueryAstNode { ColumnWrapperForUpdate (self) } }
};
}
