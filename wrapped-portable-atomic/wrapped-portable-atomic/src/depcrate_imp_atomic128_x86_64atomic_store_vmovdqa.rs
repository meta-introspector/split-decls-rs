// Generated macro for atomic_store_vmovdqa (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_store_vmovdqa {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_store_vmovdqa"}
// Dependencies: {}
# [cfg (not (any (portable_atomic_no_outline_atomics , target_env = "sgx")))] # [cfg (target_feature = "sse")] # [target_feature (enable = "avx")] # [inline] unsafe fn atomic_store_vmovdqa (dst : * mut u128 , val : u128 , order : Ordering) { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_vmovdqa_atomic ! () ; unsafe { let val : core :: arch :: x86_64 :: __m128i = core :: mem :: transmute (val) ; match order { Ordering :: Relaxed | Ordering :: Release => { asm ! (concat ! ("vmovdqa xmmword ptr [{dst" , ptr_modifier ! () , "}], {val}") , dst = in (reg) dst , val = in (xmm_reg) val , options (nostack , preserves_flags) ,) ; } Ordering :: SeqCst => { let p = core :: cell :: UnsafeCell :: new (core :: mem :: MaybeUninit :: < u64 > :: uninit ()) ; asm ! (concat ! ("vmovdqa xmmword ptr [{dst" , ptr_modifier ! () , "}], {val}") , concat ! ("xchg qword ptr [{p" , ptr_modifier ! () , "}], {tmp}") , dst = in (reg) dst , val = in (xmm_reg) val , p = inout (reg) p . get () => _ , tmp = lateout (reg) _ , options (nostack , preserves_flags) ,) ; } _ => unreachable ! () , } } }
};
}
