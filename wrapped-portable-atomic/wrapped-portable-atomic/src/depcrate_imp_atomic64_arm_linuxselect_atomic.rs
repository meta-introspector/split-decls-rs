// Generated macro for select_atomic (macro)
macro_rules! Depcrate_imp_atomic64_arm_linuxselect_atomic {
() => {
// Module: crate::imp::atomic64::arm_linux
// Provides: {"select_atomic"}
// Dependencies: {}
macro_rules ! select_atomic { (unsafe fn $ name : ident ($ dst : ident : * mut u64 $ (, $ ($ arg : tt) *) ?) $ (-> $ ret_ty : ty) ? { |$ kuser_cmpxchg64_fn_binding : ident | $ ($ kuser_cmpxchg64_fn_body : tt) * } fallback = $ seqcst_fallback_fn : ident) => { # [inline] unsafe fn $ name ($ dst : * mut u64 $ (, $ ($ arg) *) ?, _ : Ordering) $ (-> $ ret_ty) ? { unsafe fn kuser_cmpxchg64_fn ($ dst : * mut u64 $ (, $ ($ arg) *) ?) $ (-> $ ret_ty) ? { debug_assert ! ($ dst as usize % 8 == 0) ; debug_assert ! (has_kuser_cmpxchg64 ()) ; unsafe { loop { let prev = byte_wise_atomic_load ($ dst) ; let next = { let $ kuser_cmpxchg64_fn_binding = prev ; $ ($ kuser_cmpxchg64_fn_body) * } ; if __kuser_cmpxchg64 (& prev , & next , $ dst) { return prev ; } } } } unsafe { ifunc ! (unsafe fn ($ dst : * mut u64 $ (, $ ($ arg) *) ?) $ (-> $ ret_ty) ? { if has_kuser_cmpxchg64 () { kuser_cmpxchg64_fn } else { fallback ::$ seqcst_fallback_fn } }) } } } ; }
};
}
