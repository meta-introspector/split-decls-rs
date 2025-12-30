// Generated macro for atomic_swap_cmpxchg16b (function)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_swap_cmpxchg16b {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_swap_cmpxchg16b"}
// Dependencies: {}
# [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] # [inline] unsafe fn atomic_swap_cmpxchg16b (dst : * mut u128 , val : u128 , _order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_cmpxchg16b ! () ; unsafe { let val = U128 { whole : val } ; let (mut prev_lo , mut prev_hi) ; macro_rules ! cmpxchg16b { ($ rdi : tt) => { asm ! ("xchg {rbx_tmp}, rbx" , concat ! ("mov rax, qword ptr [" , $ rdi , "]") , concat ! ("mov rdx, qword ptr [" , $ rdi , " + 8]") , "2:" , concat ! ("lock cmpxchg16b xmmword ptr [" , $ rdi , "]") , "jne 2b" , "mov rbx, {rbx_tmp}" , rbx_tmp = inout (reg) val . pair . lo => _ , in ("rcx") val . pair . hi , out ("rax") prev_lo , out ("rdx") prev_hi , in ($ rdi) dst , options (nostack) ,) } ; } # [cfg (target_pointer_width = "32")] cmpxchg16b ! ("edi") ; # [cfg (target_pointer_width = "64")] cmpxchg16b ! ("rdi") ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } }
};
}
