// Generated macro for cmpxchg16b (function)
macro_rules! Depcrate_imp_atomic128_x86_64cmpxchg16b {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"cmpxchg16b"}
// Dependencies: {}
# [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] # [inline] unsafe fn cmpxchg16b (dst : * mut u128 , old : u128 , new : u128) -> (u128 , bool) { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_cmpxchg16b ! () ; unsafe { let r : u8 ; let old = U128 { whole : old } ; let new = U128 { whole : new } ; let (prev_lo , prev_hi) ; macro_rules ! cmpxchg16b { ($ rdi : tt) => { asm ! ("xchg {rbx_tmp}, rbx" , concat ! ("lock cmpxchg16b xmmword ptr [" , $ rdi , "]") , "sete cl" , "mov rbx, {rbx_tmp}" , rbx_tmp = inout (reg) new . pair . lo => _ , in ("rcx") new . pair . hi , inout ("rax") old . pair . lo => prev_lo , inout ("rdx") old . pair . hi => prev_hi , in ($ rdi) dst , lateout ("cl") r , options (nostack) ,) } ; } # [cfg (target_pointer_width = "32")] cmpxchg16b ! ("edi") ; # [cfg (target_pointer_width = "64")] cmpxchg16b ! ("rdi") ; crate :: utils :: assert_unchecked (r == 0 || r == 1) ; (U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole , r != 0) } }
};
}
