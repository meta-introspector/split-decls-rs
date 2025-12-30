// Generated macro for AssignmentTarget (trait)
macro_rules! Depcrate_query_builder_update_statement_changesetAssignmentTarget {
() => {
// Module: crate::query_builder::update_statement::changeset
// Provides: {"AssignmentTarget"}
// Dependencies: {}
# [doc = " Represents the left hand side of an assignment expression for an"] # [doc = " assignment in [AsChangeset]. The vast majority of the time, this will"] # [doc = " be a [Column]. However, in certain database backends, it's possible to"] # [doc = " assign to an expression. For example, in Postgres, it's possible to"] # [doc = " \"UPDATE TABLE SET array_column\\[1\\] = 'foo'\"."] pub trait AssignmentTarget { # [doc = " Table the assignment is to"] type Table : Table ; # [doc = " A wrapper around a type to assign to (this wrapper should implement"] # [doc = " [QueryFragment])."] type QueryAstNode ; # [doc = " Move this in to the AST node which should implement [QueryFragment]."] fn into_target (self) -> Self :: QueryAstNode ; }
};
}
