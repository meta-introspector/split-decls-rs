// Generated macro for x86 (module)
macro_rules! Depcrate_impx86 {
() => {
// Module: crate::imp
// Provides: {"x86"}
// Dependencies: {}
# [cfg (all (any (target_arch = "x86" , target_arch = "x86_64") , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) ,))] mod x86 ;
};
}
