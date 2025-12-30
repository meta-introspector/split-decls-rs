// Generated macro for atomic_load_cmpxchg16b (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_load_cmpxchg16b {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_load_cmpxchg16b"}
// Dependencies: {}
# [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] # [inline] unsafe fn atomic_load_cmpxchg16b (src : * mut u128) -> u128 { debug_assert ! (src as usize % 16 == 0) ; debug_assert_cmpxchg16b ! () ; unsafe { let (out_lo , out_hi) ; macro_rules ! cmpxchg16b { ($ rdi : tt) => { asm ! ("mov {rbx_tmp}, rbx" , "xor rbx, rbx" , concat ! ("lock cmpxchg16b xmmword ptr [" , $ rdi , "]") , "mov rbx, {rbx_tmp}" , rbx_tmp = out (reg) _ , in ("rcx") 0_u64 , inout ("rax") 0_u64 => out_lo , inout ("rdx") 0_u64 => out_hi , in ($ rdi) src , options (nostack) ,) } ; } # [cfg (target_pointer_width = "32")] cmpxchg16b ! ("edi") ; # [cfg (target_pointer_width = "64")] cmpxchg16b ! ("rdi") ; U128 { pair : Pair { lo : out_lo , hi : out_hi } } . whole } }
};
}
