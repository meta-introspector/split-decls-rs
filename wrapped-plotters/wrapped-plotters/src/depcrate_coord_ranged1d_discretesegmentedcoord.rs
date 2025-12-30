// Generated macro for SegmentedCoord (struct)
macro_rules! Depcrate_coord_ranged1d_discreteSegmentedCoord {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"SegmentedCoord"}
// Dependencies: {}
# [doc = " A `SegmentedCoord` is a decorator on any discrete coordinate specification."] # [doc = " This decorator will convert the discrete coordinate in two ways:"] # [doc = " - Add an extra dummy element after all the values in original discrete coordinate"] # [doc = " - Logically each value `v` from original coordinate system is mapped into an segment `[v, v+1)` where `v+1` denotes the successor of the `v`"] # [doc = " - Introduce two types of values `SegmentValue::Exact(value)` which denotes the left end of value's segment and `SegmentValue::CenterOf(value)` which refers the center of the segment."] # [doc = "   This is used in histogram types, which uses a discrete coordinate as the buckets."] # [doc = "   The segmented coord always emits `CenterOf(value)` key points, thus it allows all the label and tick marks"] # [doc = "   of the coordinate rendered in the middle of each segment."] # [doc = "   The corresponding trait [IntoSegmentedCoord](trait.IntoSegmentedCoord.html) is used to apply this decorator to coordinates."] # [derive (Clone)] pub struct SegmentedCoord < D : DiscreteRanged > (D) ;
};
}
