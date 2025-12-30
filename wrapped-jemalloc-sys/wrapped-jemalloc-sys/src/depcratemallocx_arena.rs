// Generated macro for MALLOCX_ARENA (function)
macro_rules! DepcrateMALLOCX_ARENA {
() => {
// Module: crate
// Provides: {"MALLOCX_ARENA"}
// Dependencies: {}
# [doc = " Use the arena specified by the index `a`."] # [doc = ""] # [doc = " This option has no effect for regions that were allocated via an arena other"] # [doc = " than the one specified."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function does not validate that `a` specifies an arena index in the"] # [doc = " valid range."] # [inline] pub const fn MALLOCX_ARENA (a : usize) -> c_int { (a as c_int) . wrapping_add (1) . wrapping_shl (20) }
};
}
