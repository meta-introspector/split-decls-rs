// Generated macro for MaybeEmpty (trait)
macro_rules! Depcrate_expression_array_comparisonMaybeEmpty {
() => {
// Module: crate::expression::array_comparison
// Provides: {"MaybeEmpty"}
// Dependencies: {}
# [doc = " A helper trait to check if the values clause of"] # [doc = " an [`In`] or [`NotIn`] query dsl node is empty or not"] pub trait MaybeEmpty { # [doc = " Returns `true` if self represents an empty collection"] # [doc = " Otherwise `false` is returned."] fn is_empty (& self) -> bool ; }
};
}
