// Generated macro for ReversibleRanged (trait)
macro_rules! Depcrate_coord_ranged1dReversibleRanged {
() => {
// Module: crate::coord::ranged1d
// Provides: {"ReversibleRanged"}
// Dependencies: {}
# [doc = " The trait indicates the ranged value can be map reversely, which means"] # [doc = " an pixel-based coordinate is given, it's possible to figure out the underlying"] # [doc = " logic value."] pub trait ReversibleRanged : Ranged { # [doc = " Perform the reverse mapping"] fn unmap (& self , input : i32 , limit : (i32 , i32)) -> Option < Self :: ValueType > ; }
};
}
