// Generated macro for w (macro)
macro_rules! Depcrate_imp_riscvw {
() => {
// Module: crate::imp::riscv
// Provides: {"w"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] # [cfg (target_arch = "riscv64")] macro_rules ! w { () => { "w" } ; }
};
}
