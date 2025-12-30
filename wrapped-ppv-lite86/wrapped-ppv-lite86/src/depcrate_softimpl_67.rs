// Generated macro for impl_67 (impl)
macro_rules! Depcrate_softimpl_67 {
() => {
// Module: crate::soft
// Provides: {"impl_67"}
// Dependencies: {}
impl < W : Copy > MultiLane < [W ; 4] > for x4 < W > { # [inline (always)] fn to_lanes (self) -> [W ; 4] { self . 0 } # [inline (always)] fn from_lanes (lanes : [W ; 4]) -> Self { x4 (lanes) } }
};
}
