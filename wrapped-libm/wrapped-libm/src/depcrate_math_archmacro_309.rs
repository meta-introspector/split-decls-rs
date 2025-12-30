// Generated macro for macro_309 (macro)
macro_rules! Depcrate_math_archmacro_309 {
() => {
// Module: crate::math::arch
// Provides: {"macro_309"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] { mod i586 ; pub use i586 :: { ceil , floor } ; } }
};
}
