// Generated macro for SegmentValue (enum)
macro_rules! Depcrate_coord_ranged1d_discreteSegmentValue {
() => {
// Module: crate::coord::ranged1d::discrete
// Provides: {"SegmentValue"}
// Dependencies: {}
# [doc = " The value that used by the segmented coordinate."] # [derive (Clone , Debug)] pub enum SegmentValue < T > { # [doc = " Means we are referring the exact position of value `T`"] Exact (T) , # [doc = " Means we are referring the center of position `T` and the successor of `T`"] CenterOf (T) , # [doc = " Referring the last dummy element"] Last , }
};
}
