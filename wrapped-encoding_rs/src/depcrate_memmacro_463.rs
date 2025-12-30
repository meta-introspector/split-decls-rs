// Generated macro for macro_463 (macro)
macro_rules! Depcrate_memmacro_463 {
() => {
// Module: crate::mem
// Provides: {"macro_463"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "simd-accel")] { use :: core :: intrinsics :: likely ; use :: core :: intrinsics :: unlikely ; } else { # [inline (always)] fn likely (b : bool) -> bool { b } # [inline (always)] fn unlikely (b : bool) -> bool { b } } }
};
}
