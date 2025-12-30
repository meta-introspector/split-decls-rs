// Generated macro for __kuser_helper_version (function)
macro_rules! Depcrate_imp_atomic64_arm_linux__kuser_helper_version {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"__kuser_helper_version"}
// Dependencies: {}
# [inline] fn __kuser_helper_version () -> i32 { use core :: sync :: atomic :: AtomicI32 ; static CACHE : AtomicI32 = AtomicI32 :: new (0) ; let mut v = CACHE . load (Ordering :: Relaxed) ; if v != 0 { return v ; } v = unsafe { crate :: utils :: ptr :: with_exposed_provenance :: < i32 > (KUSER_HELPER_VERSION) . read () } ; CACHE . store (v , Ordering :: Relaxed) ; v }
};
}
