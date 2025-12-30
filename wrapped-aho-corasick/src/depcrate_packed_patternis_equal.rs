// Generated macro for is_equal (function)
macro_rules! Depcrate_packed_patternis_equal {
() => {
// Module: crate::packed::pattern
// Provides: {"is_equal"}
// Dependencies: {}
# [doc = " Compare corresponding bytes in `x` and `y` for equality."] # [doc = ""] # [doc = " That is, this returns true if and only if `x.len() == y.len()` and"] # [doc = " `x[i] == y[i]` for all `0 <= i < x.len()`."] # [doc = ""] # [doc = " Note that this isn't used. We only use it in tests as a convenient way"] # [doc = " of testing `is_equal_raw`."] # [doc = ""] # [doc = " # Inlining"] # [doc = ""] # [doc = " This routine is marked `inline(always)`. If you want to call this function"] # [doc = " in a way that is not always inlined, you'll need to wrap a call to it in"] # [doc = " another function that is marked as `inline(never)` or just `inline`."] # [doc = ""] # [doc = " # Motivation"] # [doc = ""] # [doc = " Why not use slice equality instead? Well, slice equality usually results in"] # [doc = " a call out to the current platform's `libc` which might not be inlineable"] # [doc = " or have other overhead. This routine isn't guaranteed to be a win, but it"] # [doc = " might be in some cases."] # [cfg (test)] # [inline (always)] fn is_equal (x : & [u8] , y : & [u8]) -> bool { if x . len () != y . len () { return false ; } unsafe { is_equal_raw (x . as_ptr () , y . as_ptr () , x . len ()) } }
};
}
