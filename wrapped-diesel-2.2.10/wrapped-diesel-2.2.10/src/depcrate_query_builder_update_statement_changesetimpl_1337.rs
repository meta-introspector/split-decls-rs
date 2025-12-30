// Generated macro for impl_1337 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1337 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1337"}
// Dependencies: {}
impl < T : AsChangeset > AsChangeset for Option < T > { type Target = T :: Target ; type Changeset = Option < T :: Changeset > ; fn as_changeset (self) -> Self :: Changeset { self . map (AsChangeset :: as_changeset) } }
};
}
