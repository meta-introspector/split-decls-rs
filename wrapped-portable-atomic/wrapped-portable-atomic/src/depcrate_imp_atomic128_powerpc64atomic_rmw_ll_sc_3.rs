// Generated macro for atomic_rmw_ll_sc_3 (macro)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_rmw_ll_sc_3 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_rmw_ll_sc_3"}
// Dependencies: {}
# [doc = " Atomic RMW by LL/SC loop (3 arguments)"] # [doc = " `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`"] # [doc = ""] # [doc = " $op can use the following registers:"] # [doc = " - val_hi/val_lo pair: val argument (read-only for `$op`)"] # [doc = " - r6/r7 pair: previous value loaded by ll (read-only for `$op`)"] # [doc = " - r8/r9 pair: new value that will be stored by sc"] macro_rules ! atomic_rmw_ll_sc_3 { ($ name : ident , [$ ($ reg : tt) *] , $ ($ op : tt) *) => { # [inline] unsafe fn $ name (dst : * mut u128 , val : u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_pwr8 ! () ; let val = U128 { whole : val } ; let (mut prev_hi , mut prev_lo) ; unsafe { macro_rules ! op { ($ acquire : tt , $ release : tt) => { asm ! (start_pwr8 ! () , $ release , "2:" , "lqarx %r6, 0, {dst}" , $ ($ op) * "stqcx. %r8, 0, {dst}" , "bne %cr0, 2b" , $ acquire , end_pwr8 ! () , dst = in (reg_nonzero) ptr_reg ! (dst) , val_hi = in (reg) val . pair . hi , val_lo = in (reg) val . pair . lo , $ ($ reg) * out ("r6") prev_hi , out ("r7") prev_lo , out ("r8") _ , out ("r9") _ , out ("cr0") _ , options (nostack , preserves_flags) ,) } ; } atomic_rmw ! (op , order) ; U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole } } } ; }
};
}
