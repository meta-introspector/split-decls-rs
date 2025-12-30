// Generated macro for macro_14 (macro)
macro_rules! Depcrate_float_cmpmacro_14 {
() => {
// Module: crate::float::cmp
// Provides: {"macro_14"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (target_arch = "aarch64" , target_arch = "arm64ec"))] { pub type CmpResult = i32 ; } else if # [cfg (target_arch = "avr")] { pub type CmpResult = i8 ; } else { pub type CmpResult = isize ; } }
};
}
