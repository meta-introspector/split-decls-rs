// Generated macro for u32_min (function)
macro_rules! Depcrate_const_choiceu32_min {
() => {
// Module: crate::const_choice
// Provides: {"u32_min"}
// Dependencies: {}
# [doc = " `const` equivalent of `u32::min(a, b)`."] pub const fn u32_min (a : u32 , b : u32) -> u32 { ConstChoice :: from_u32_lt (a , b) . select_u32 (b , a) }
};
}
