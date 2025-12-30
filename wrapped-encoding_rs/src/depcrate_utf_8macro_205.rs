// Generated macro for macro_205 (macro)
macro_rules! Depcrate_utf_8macro_205 {
() => {
// Module: crate::utf_8
// Provides: {"macro_205"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "simd-accel")] { use :: core :: intrinsics :: unlikely ; use :: core :: intrinsics :: likely ; } else { # [inline (always)] fn unlikely (b : bool) -> bool { b } # [inline (always)] fn likely (b : bool) -> bool { b } } }
};
}
