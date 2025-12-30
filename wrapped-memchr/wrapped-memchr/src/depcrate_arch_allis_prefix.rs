// Generated macro for is_prefix (function)
macro_rules! Depcrate_arch_allis_prefix {
() => {
// Module: crate::arch::all
// Provides: {"is_prefix"}
// Dependencies: {}
# [doc = " Returns true if and only if `needle` is a prefix of `haystack`."] # [doc = ""] # [doc = " This uses a latency optimized variant of `memcmp` internally which *might*"] # [doc = " make this faster for very short strings."] # [doc = ""] # [doc = " # Inlining"] # [doc = ""] # [doc = " This routine is marked `inline(always)`. If you want to call this function"] # [doc = " in a way that is not always inlined, you'll need to wrap a call to it in"] # [doc = " another function that is marked as `inline(never)` or just `inline`."] # [inline (always)] pub fn is_prefix (haystack : & [u8] , needle : & [u8]) -> bool { needle . len () <= haystack . len () && is_equal (& haystack [.. needle . len ()] , needle) }
};
}
