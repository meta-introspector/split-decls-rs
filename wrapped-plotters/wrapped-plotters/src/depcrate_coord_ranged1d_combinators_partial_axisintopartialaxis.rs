// Generated macro for IntoPartialAxis (trait)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axisIntoPartialAxis {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"IntoPartialAxis"}
// Dependencies: {}
# [doc = " The trait for the types that can be converted into a partial axis"] pub trait IntoPartialAxis : AsRangedCoord { # [doc = " Make the partial axis"] # [doc = ""] # [doc = " - `axis_range`: The range of the axis to be displayed"] # [doc = " - **returns**: The converted range specification"] fn partial_axis (self , axis_range : Range < < Self :: CoordDescType as Ranged > :: ValueType > ,) -> PartialAxis < Self :: CoordDescType > { PartialAxis (self . into () , axis_range) } }
};
}
