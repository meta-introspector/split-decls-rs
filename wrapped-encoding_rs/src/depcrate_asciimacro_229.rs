// Generated macro for macro_229 (macro)
macro_rules! Depcrate_asciimacro_229 {
() => {
// Module: crate::ascii
// Provides: {"macro_229"}
// Dependencies: {}
cfg_if ! { if # [cfg (feature = "simd-accel")] { # [allow (unused_imports)] use :: core :: intrinsics :: unlikely ; # [allow (unused_imports)] use :: core :: intrinsics :: likely ; } else { # [allow (dead_code)] # [inline (always)] fn unlikely (b : bool) -> bool { b } # [allow (dead_code)] # [inline (always)] fn likely (b : bool) -> bool { b } } }
};
}
