// Generated macro for impl_1339 (impl)
macro_rules! Depcrate_query_builder_update_statement_changesetimpl_1339 {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"impl_1339"}
// Dependencies: {}
impl < Left , Right > AsChangeset for Grouped < Eq < Left , Right > > where Eq < Left , Right > : AsChangeset , { type Target = < Eq < Left , Right > as AsChangeset > :: Target ; type Changeset = < Eq < Left , Right > as AsChangeset > :: Changeset ; fn as_changeset (self) -> Self :: Changeset { self . 0 . as_changeset () } }
};
}
