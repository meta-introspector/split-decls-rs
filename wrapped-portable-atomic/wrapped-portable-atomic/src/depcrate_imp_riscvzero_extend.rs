// Generated macro for zero_extend (macro)
macro_rules! Depcrate_imp_riscvzero_extend {
() => {
// Module: crate::imp::riscv
// Provides: {"zero_extend"}
// Dependencies: {}
macro_rules ! zero_extend { ($ int : ident , $ uint : ident) => { # [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] impl ZeroExtend for $ uint { # [inline (always)] fn zero_extend (self) -> u32 { self as u32 } } # [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] impl ZeroExtend for $ int { # [allow (clippy :: cast_sign_loss)] # [inline (always)] fn zero_extend (self) -> u32 { self as $ uint as u32 } } } ; }
};
}
