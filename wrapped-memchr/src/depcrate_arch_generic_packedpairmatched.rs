// Generated macro for matched (function)
macro_rules! Depcrate_arch_generic_packedpairmatched {
() => {
// Module: crate::arch::generic::packedpair
// Provides: {"matched"}
// Dependencies: {}
# [doc = " Accepts a chunk-relative offset and returns a haystack relative offset."] # [doc = ""] # [doc = " This used to be marked `#[cold]` and `#[inline(never)]`, but I couldn't"] # [doc = " observe a consistent measureable difference between that and just inlining"] # [doc = " it. So we go with inlining it."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same at `ptr::offset_from` in addition to `cur >= start`."] # [inline (always)] unsafe fn matched (start : * const u8 , cur : * const u8 , chunki : usize) -> usize { cur . distance (start) + chunki }
};
}
