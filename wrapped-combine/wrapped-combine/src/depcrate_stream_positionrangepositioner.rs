// Generated macro for RangePositioner (trait)
macro_rules! Depcrate_stream_positionRangePositioner {
() => {
// Module: crate::stream::position
// Provides: {"RangePositioner"}
// Dependencies: {}
# [doc = " Trait for tracking the current position of a `RangeStream`."] pub trait RangePositioner < Item , Range > : Positioner < Item > { # [doc = " Updates the position given that `range` has been taken from the stream"] fn update_range (& mut self , range : & Range) ; }
};
}
