// Generated macro for AsChangeset (trait)
macro_rules! Depcrate_query_builder_update_statement_changesetAsChangeset {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"AsChangeset"}
// Dependencies: {}
# [doc = " Types which can be passed to"] # [doc = " [`update.set`](UpdateStatement::set())."] # [doc = ""] # [doc = " This trait can be [derived](derive@AsChangeset)"] pub trait AsChangeset { # [doc = " The table which `Self::Changeset` will be updating"] type Target : QuerySource ; # [doc = " The update statement this type represents"] type Changeset ; # [doc = " Convert `self` into the actual update statement being executed"] # [allow (clippy :: wrong_self_convention)] fn as_changeset (self) -> Self :: Changeset ; }
};
}
