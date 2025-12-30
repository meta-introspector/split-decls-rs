// Generated macro for atomic_rmw_cas_2 (macro)
macro_rules! Depcrate_imp_atomic128_aarch64atomic_rmw_cas_2 {
() => {
// Module: crate::imp::atomic128::aarch64
// Provides: {"atomic_rmw_cas_2"}
// Dependencies: {}
# [doc = " Atomic RMW by CAS loop (2 arguments)"] # [doc = " `unsafe fn(dst: *mut u128, order: Ordering) -> u128;`"] # [doc = ""] # [doc = " `$op` can use the following registers:"] # [doc = " - x6/x7 pair: previous value loaded (read-only for `$op`)"] # [doc = " - x4/x5 pair: new value that will be stored"] macro_rules ! atomic_rmw_cas_2 { ($ name : ident as $ reexport_name : ident , $ ($ op : tt) *) => { # [cfg (all (any (target_feature = "lse" , portable_atomic_target_feature = "lse") , not (portable_atomic_ll_sc_rmw) ,))] use self ::$ name as $ reexport_name ; # [cfg (any (test , not (portable_atomic_ll_sc_rmw)))] # [cfg (any (target_feature = "lse" , portable_atomic_target_feature = "lse"))] # [inline] unsafe fn $ name (dst : * mut u128 , order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_lse ! () ; unsafe { let (mut prev_lo , mut prev_hi) ; macro_rules ! op { ($ acquire : tt , $ release : tt , $ fence : tt) => { asm ! (start_lse ! () , "ldp x6, x7, [{dst}]" , "2:" , "mov {tmp_lo}, x6" , "mov {tmp_hi}, x7" , $ ($ op) * concat ! ("casp" , $ acquire , $ release , " x6, x7, x4, x5, [{dst}]") , "cmp {tmp_hi}, x7" , "ccmp {tmp_lo}, x6, #0, eq" , "b.ne 2b" , $ fence , dst = in (reg) ptr_reg ! (dst) , tmp_lo = out (reg) _ , tmp_hi = out (reg) _ , out ("x6") prev_lo , out ("x7") prev_hi , out ("x4") _ , out ("x5") _ , options (nostack) ,) } ; } atomic_rmw ! (op , order) ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } } } ; }
};
}
