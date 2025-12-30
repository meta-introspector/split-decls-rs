// Generated macro for NestedRange (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_nestedNestedRange {
() => {
// Module: crate::coord::ranged1d::combinators::nested
// Provides: {"NestedRange"}
// Dependencies: {}
# [doc = " A nested coordinate spec which is a discrete coordinate on the top level and"] # [doc = " for each value in discrete value, there is a secondary coordinate system."] # [doc = " And the value is defined as a tuple of primary coordinate value and secondary"] # [doc = " coordinate value"] # [derive (Clone)] pub struct NestedRange < Primary : DiscreteRanged , Secondary : Ranged > { primary : Primary , secondary : Vec < Secondary > , }
};
}
