// Generated macro for LinspaceRoundingMethod (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceLinspaceRoundingMethod {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"LinspaceRoundingMethod"}
// Dependencies: {}
# [doc = " The type marker used to denote the rounding method."] # [doc = " Since we are mapping any range to a discrete range thus not all values are"] # [doc = " perfect mapped to the grid points. In this case, this type marker gives hints"] # [doc = " for the linspace coord for how to treat the non-grid-point values."] pub trait LinspaceRoundingMethod < V > { # [doc = " Search for the value within the given values array and rounding method"] # [doc = ""] # [doc = " - `values`: The values we want to search"] # [doc = " - `target`: The target value"] # [doc = " - `returns`: The index if we found the matching item, otherwise none"] fn search (values : & [V] , target : & V) -> Option < usize > ; }
};
}
