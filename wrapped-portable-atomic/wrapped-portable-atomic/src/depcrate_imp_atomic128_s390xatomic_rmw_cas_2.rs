// Generated macro for atomic_rmw_cas_2 (macro)
macro_rules! Depcrate_imp_atomic128_s390xatomic_rmw_cas_2 {
() => {
// Module: crate::imp::atomic128::s390x
// Provides: {"atomic_rmw_cas_2"}
// Dependencies: {}
# [doc = " Atomic RMW by CAS loop (2 arguments)"] # [doc = " `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`"] # [doc = ""] # [doc = " `$op` can use the following registers:"] # [doc = " - r0/r1 pair: previous value loaded (read-only for `$op`)"] # [doc = " - r12/r13 pair: new value that will be stored"] macro_rules ! atomic_rmw_cas_2 { ($ name : ident , [$ ($ reg : tt) *] , $ ($ op : tt) *) => { # [inline] unsafe fn $ name (dst : * mut u128 , _order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; let (mut prev_hi , mut prev_lo) ; unsafe { asm ! ("lg %r0, 8({dst})" , "lg %r1, 0({dst})" , "2:" , $ ($ op) * "cdsg %r0, %r12, 0({dst})" , "jl 2b" , dst = in (reg) ptr_reg ! (dst) , $ ($ reg) * out ("r0") prev_hi , out ("r1") prev_lo , out ("r12") _ , out ("r13") _ , options (nostack) ,) ; U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole } } } ; }
};
}
