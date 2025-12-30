// Generated macro for ZeroExtend (trait)
macro_rules! Depcrate_imp_riscvZeroExtend {
() => {
// Module: crate::imp::riscv
// Provides: {"ZeroExtend"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] # [cfg (not (any (target_feature = "zabha" , portable_atomic_target_feature = "zabha")))] trait ZeroExtend : Copy { # [doc = " Zero-extends `self` to `u32` if it is smaller than 32-bit."] fn zero_extend (self) -> u32 ; }
};
}
