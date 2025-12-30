// Generated macro for srlw (macro)
macro_rules! Depcrate_imp_riscvsrlw {
() => {
// Module: crate::imp::riscv
// Provides: {"srlw"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] macro_rules ! srlw { ($ val : expr , $ shift : expr) => { unsafe { let val : u32 = $ val ; let shift : u32 = $ shift ; let out ; asm ! (concat ! ("srl" , w ! () , " {out}, {val}, {shift}") , out = lateout (reg) out , val = in (reg) val , shift = in (reg) shift , options (pure , nomem , nostack , preserves_flags) ,) ; out } } ; }
};
}
