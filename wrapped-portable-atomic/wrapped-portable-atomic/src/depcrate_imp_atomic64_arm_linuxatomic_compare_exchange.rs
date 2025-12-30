// Generated macro for atomic_compare_exchange (function)
macro_rules! Depcrate_imp_atomic64_arm_linuxatomic_compare_exchange {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"atomic_compare_exchange"}
// Dependencies: {}
# [inline] unsafe fn atomic_compare_exchange (dst : * mut u64 , old : u64 , new : u64 , _ : Ordering , _ : Ordering ,) -> Result < u64 , u64 > { unsafe fn kuser_cmpxchg64_fn (dst : * mut u64 , old : u64 , new : u64) -> (u64 , bool) { debug_assert ! (dst as usize % 8 == 0) ; debug_assert ! (has_kuser_cmpxchg64 ()) ; unsafe { loop { let prev = byte_wise_atomic_load (dst) ; let next = if prev == old { new } else { prev } ; if __kuser_cmpxchg64 (& prev , & next , dst) { return (prev , prev == old) ; } } } } let (prev , ok) = unsafe { ifunc ! (unsafe fn (dst : * mut u64 , old : u64 , new : u64) -> (u64 , bool) { if has_kuser_cmpxchg64 () { kuser_cmpxchg64_fn } else { fallback :: atomic_compare_exchange_seqcst } }) } ; if ok { Ok (prev) } else { Err (prev) } }
};
}
