// Generated macro for __kuser_cmpxchg64 (function)
macro_rules! Depcrate_imp_atomic64_arm_linux__kuser_cmpxchg64 {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"__kuser_cmpxchg64"}
// Dependencies: {}
# [inline] unsafe fn __kuser_cmpxchg64 (old_val : * const u64 , new_val : * const u64 , ptr : * mut u64) -> bool { unsafe { let f : extern "C" fn (* const u64 , * const u64 , * mut u64) -> u32 = mem :: transmute (crate :: utils :: ptr :: with_exposed_provenance :: < () > (KUSER_CMPXCHG64)) ; f (old_val , new_val , ptr) == 0 } }
};
}
