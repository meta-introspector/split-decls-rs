// Generated macro for sllw (function)
macro_rules! Depcrate_imp_riscvsllw {
() => {
// Module: crate::imp::riscv
// Provides: {"sllw"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] # [inline (always)] fn sllw (val : u32 , shift : u32) -> u32 { unsafe { let out ; asm ! (concat ! ("sll" , w ! () , " {out}, {val}, {shift}") , out = lateout (reg) out , val = in (reg) val , shift = in (reg) shift , options (pure , nomem , nostack , preserves_flags) ,) ; out } }
};
}
