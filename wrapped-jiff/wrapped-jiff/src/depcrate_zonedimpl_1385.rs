// Generated macro for impl_1385 (impl)
macro_rules! Depcrate_zonedimpl_1385 {
() => {
// Module: crate::zoned
// Provides: {"impl_1385"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Zoned > for std :: time :: SystemTime { # [inline] fn from (time : Zoned) -> std :: time :: SystemTime { time . timestamp () . into () } }
};
}
