// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
# [cfg (portable_atomic_unsafe_assume_single_core)] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (portable_atomic_no_atomic_cas)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (target_has_atomic = "ptr"))] compile_error ! ("`portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) \
     is not compatible with target that supports atomic CAS;\n\
     see also <https://github.com/taiki-e/portable-atomic/issues/148> for troubleshooting") ;
};
}
