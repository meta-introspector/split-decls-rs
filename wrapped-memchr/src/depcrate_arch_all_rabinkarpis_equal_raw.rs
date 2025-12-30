// Generated macro for is_equal_raw (function)
macro_rules! Depcrate_arch_all_rabinkarpis_equal_raw {
() => {
// Module: crate::arch::all::rabinkarp
// Provides: {"is_equal_raw"}
// Dependencies: {}
# [doc = " Returns true when `x[i] == y[i]` for all `0 <= i < n`."] # [doc = ""] # [doc = " We forcefully don't inline this to hint at the compiler that it is unlikely"] # [doc = " to be called. This causes the inner rabinkarp loop above to be a bit"] # [doc = " tighter and leads to some performance improvement. See the"] # [doc = " memmem/krate/prebuilt/sliceslice-words/words benchmark."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Same as `crate::arch::all::is_equal_raw`."] # [cold] # [inline (never)] unsafe fn is_equal_raw (x : * const u8 , y : * const u8 , n : usize) -> bool { crate :: arch :: all :: is_equal_raw (x , y , n) }
};
}
