// Generated macro for impl_1338 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1338 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1338"}
// Dependencies: {}
impl < Left , Right > AsChangeset for Eq < Left , Right > where Left : AssignmentTarget , Right : AppearsOnTable < Left :: Table > , { type Target = Left :: Table ; type Changeset = Assign < < Left as AssignmentTarget > :: QueryAstNode , Right > ; fn as_changeset (self) -> Self :: Changeset { Assign { target : self . left . into_target () , expr : self . right , } } }
};
}
