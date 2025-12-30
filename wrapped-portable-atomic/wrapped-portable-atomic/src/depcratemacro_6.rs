// Generated macro for macro_6 (macro)
macro_rules! Depcratemacro_6 {
() => {
// Module: crate
// Provides: {"macro_6"}
// Dependencies: {}
# [cfg (portable_atomic_disable_fiq)] # [cfg (not (all (target_arch = "arm" , not (any (target_feature = "mclass" , portable_atomic_target_feature = "mclass")) ,)))] compile_error ! ("`portable_atomic_disable_fiq` cfg (`disable-fiq` feature) is only available on pre-v6 Arm") ;
};
}
