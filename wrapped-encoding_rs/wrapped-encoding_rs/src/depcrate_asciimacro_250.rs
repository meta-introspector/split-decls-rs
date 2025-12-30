// Generated macro for macro_250 (macro)
macro_rules! Depcrate_asciimacro_250 {
() => {
// Module: crate::ascii
// Provides: {"macro_250"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_endian = "little")] { # [allow (dead_code)] # [inline (always)] fn count_zeros (word : usize) -> u32 { word . trailing_zeros () } } else { # [allow (dead_code)] # [inline (always)] fn count_zeros (word : usize) -> u32 { word . leading_zeros () } } }
};
}
