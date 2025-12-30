// Generated macro for impl_1386 (impl)
macro_rules! Depcrate_zonedimpl_1386 {
() => {
// Module: crate::zoned
// Provides: {"impl_1386"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > From < & 'a Zoned > for std :: time :: SystemTime { # [inline] fn from (time : & 'a Zoned) -> std :: time :: SystemTime { time . timestamp () . into () } }
};
}
