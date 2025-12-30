// Generated macro for MultiLane (trait)
macro_rules! Depcrate_typesMultiLane {
() => {
// Module: crate::types
// Provides: {"MultiLane"}
// Dependencies: {}
# [doc = " A vector composed of multiple 128-bit lanes."] pub trait MultiLane < Lanes > { # [doc = " Split a multi-lane vector into single-lane vectors."] fn to_lanes (self) -> Lanes ; # [doc = " Build a multi-lane vector from individual lanes."] fn from_lanes (lanes : Lanes) -> Self ; }
};
}
