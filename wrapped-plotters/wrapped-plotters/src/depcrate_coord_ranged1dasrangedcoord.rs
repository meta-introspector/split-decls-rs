// Generated macro for AsRangedCoord (trait)
macro_rules! Depcrate_coord_ranged1dAsRangedCoord {
() => {
// Module: crate::coord::ranged1d
// Provides: {"AsRangedCoord"}
// Dependencies: {}
# [doc = " The trait for the type that can be converted into a ranged coordinate axis"] pub trait AsRangedCoord : Sized { # [doc = " Type to describe a coordinate system"] type CoordDescType : Ranged < ValueType = Self :: Value > + From < Self > ; # [doc = " Type for values in the given coordinate system"] type Value ; }
};
}
