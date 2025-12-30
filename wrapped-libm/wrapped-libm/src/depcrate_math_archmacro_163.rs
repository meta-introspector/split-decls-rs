// Generated macro for macro_163 (macro)
macro_rules! Depcrate_math_archmacro_163 {
() => {
// Module: crate::math::arch
// Provides: {"macro_163"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] { mod i586 ; pub use i586 :: { ceil , floor } ; } }
};
}
