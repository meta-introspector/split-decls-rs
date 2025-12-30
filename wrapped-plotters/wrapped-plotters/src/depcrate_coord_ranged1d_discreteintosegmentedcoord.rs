// Generated macro for IntoSegmentedCoord (trait)
macro_rules! Depcrate_coord_ranged1d_discreteIntoSegmentedCoord {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"IntoSegmentedCoord"}
// Dependencies: {}
# [doc = " The trait for types that can decorated by [SegmentedCoord](struct.SegmentedCoord.html) decorator."] pub trait IntoSegmentedCoord : AsRangedCoord where Self :: CoordDescType : DiscreteRanged , { # [doc = " Convert current ranged value into a segmented coordinate"] fn into_segmented (self) -> SegmentedCoord < Self :: CoordDescType > { SegmentedCoord (self . into ()) } }
};
}
