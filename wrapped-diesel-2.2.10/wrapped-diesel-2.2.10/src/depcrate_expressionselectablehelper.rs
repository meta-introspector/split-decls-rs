// Generated macro for SelectableHelper (trait)
macro_rules! Depcrate_expressionSelectableHelper {
() => {
// Module: crate::expression
// Provides: {"SelectableHelper"}
// Dependencies: {}
# [doc = " This helper trait provides several methods for"] # [doc = " constructing a select or returning clause based on a"] # [doc = " [`Selectable`] implementation."] pub trait SelectableHelper < DB : Backend > : Selectable < DB > + Sized { # [doc = " Construct a select clause based on a [`Selectable`] implementation."] # [doc = ""] # [doc = " The returned select clause enforces that you use the same type"] # [doc = " for constructing the select clause and for loading the query result into."] fn as_select () -> AsSelect < Self , DB > ; # [doc = " An alias for `as_select` that can be used with returning clauses"] fn as_returning () -> AsSelect < Self , DB > { Self :: as_select () } }
};
}
