// Generated macro for u32_max (function)
macro_rules! Depcrate_const_choiceu32_max {
() => {
// Module: crate::const_choice
// Provides: {"u32_max"}
// Dependencies: {}
# [doc = " `const` equivalent of `u32::max(a, b)`."] pub const fn u32_max (a : u32 , b : u32) -> u32 { ConstChoice :: from_u32_lt (a , b) . select_u32 (a , b) }
};
}
