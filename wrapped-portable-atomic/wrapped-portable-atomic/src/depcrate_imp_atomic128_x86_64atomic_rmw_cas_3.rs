// Generated macro for atomic_rmw_cas_3 (macro)
macro_rules! Depcrate_imp_atomic128_x86_64atomic_rmw_cas_3 {
() => {
// Module: crate::imp::atomic128::x86_64
// Provides: {"atomic_rmw_cas_3"}
// Dependencies: {}
# [doc = " Atomic RMW by CAS loop (3 arguments)"] # [doc = " `unsafe fn(dst: *mut u128, val: u128, order: Ordering) -> u128;`"] # [doc = ""] # [doc = " `$op` can use the following registers:"] # [doc = " - rsi/r8 pair: val argument (read-only for `$op`)"] # [doc = " - rax/rdx pair: previous value loaded (read-only for `$op`)"] # [doc = " - rbx/rcx pair: new value that will be stored"] macro_rules ! atomic_rmw_cas_3 { ($ name : ident , $ ($ op : tt) *) => { # [cfg_attr (not (portable_atomic_no_cmpxchg16b_target_feature) , target_feature (enable = "cmpxchg16b"))] # [inline] unsafe fn $ name (dst : * mut u128 , val : u128 , _order : Ordering) -> u128 { debug_assert ! (dst as usize % 16 == 0) ; debug_assert_cmpxchg16b ! () ; unsafe { let val = U128 { whole : val } ; let (mut prev_lo , mut prev_hi) ; macro_rules ! cmpxchg16b { ($ rdi : tt) => { asm ! ("mov {rbx_tmp}, rbx" , concat ! ("mov rax, qword ptr [" , $ rdi , "]") , concat ! ("mov rdx, qword ptr [" , $ rdi , " + 8]") , "2:" , $ ($ op) * concat ! ("lock cmpxchg16b xmmword ptr [" , $ rdi , "]") , "jne 2b" , "mov rbx, {rbx_tmp}" , rbx_tmp = out (reg) _ , out ("rcx") _ , out ("rax") prev_lo , out ("rdx") prev_hi , in ($ rdi) dst , in ("rsi") val . pair . lo , in ("r8") val . pair . hi , options (nostack) ,) } ; } # [cfg (target_pointer_width = "32")] cmpxchg16b ! ("edi") ; # [cfg (target_pointer_width = "64")] cmpxchg16b ! ("rdi") ; U128 { pair : Pair { lo : prev_lo , hi : prev_hi } } . whole } } } ; }
};
}
