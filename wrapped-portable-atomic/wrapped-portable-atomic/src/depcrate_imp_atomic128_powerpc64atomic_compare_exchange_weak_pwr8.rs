// Generated macro for atomic_compare_exchange_weak_pwr8 (function)
macro_rules! Depcrate_imp_atomic128_powerpc64atomic_compare_exchange_weak_pwr8 {
() => {
// Module: crate::imp::atomic128::powerpc64
// Provides: {"atomic_compare_exchange_weak_pwr8"}
// Dependencies: {}
# [cfg (any (target_feature = "quadword-atomics" , portable_atomic_target_feature = "quadword-atomics" ,))] # [inline] unsafe fn atomic_compare_exchange_weak_pwr8 (dst : * mut u128 , old : u128 , new : u128 , success : Ordering , failure : Ordering ,) -> (u128 , bool) { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_pwr8 ! () ; let old = U128 { whole : old } ; let new = U128 { whole : new } ; let (mut prev_hi , mut prev_lo) ; let mut r ; unsafe { macro_rules ! cmpxchg_weak { ($ acquire_always : tt , $ acquire_success : tt , $ release : tt) => { asm ! (start_pwr8 ! () , $ release , "lqarx %r8, 0, {dst}" , "xor {tmp_lo}, %r9, {old_lo}" , "xor {tmp_hi}, %r8, {old_hi}" , "or. {tmp_lo}, {tmp_lo}, {tmp_hi}" , "bne %cr0, 3f" , "stqcx. %r6, 0, {dst}" , $ acquire_success , "3:" , $ acquire_always , "mfcr {tmp_lo}" , end_pwr8 ! () , dst = in (reg_nonzero) ptr_reg ! (dst) , old_hi = in (reg) old . pair . hi , old_lo = in (reg) old . pair . lo , tmp_hi = out (reg) _ , tmp_lo = out (reg) r , in ("r6") new . pair . hi , in ("r7") new . pair . lo , out ("r8") prev_hi , out ("r9") prev_lo , out ("cr0") _ , options (nostack , preserves_flags) ,) } ; } atomic_cas ! (cmpxchg_weak , success , failure) ; (U128 { pair : Pair { hi : prev_hi , lo : prev_lo } } . whole , test_cr0_eq (r)) } }
};
}
