// Generated macro for atomic_rmw_amo_ext (macro)
macro_rules! Depcrate_imp_riscvatomic_rmw_amo_ext {
() => {
// Module: crate::imp::riscv
// Provides: {"atomic_rmw_amo_ext"}
// Dependencies: {}
# [cfg (any (test , portable_atomic_force_amo , target_feature = "zaamo" , portable_atomic_target_feature = "zaamo" ,))] macro_rules ! atomic_rmw_amo_ext { ("w") => { "+a" } ; ("d") => { "+a" } ; ("b") => { "+a,+zabha" } ; ("h") => { "+a,+zabha" } ; }
};
}
