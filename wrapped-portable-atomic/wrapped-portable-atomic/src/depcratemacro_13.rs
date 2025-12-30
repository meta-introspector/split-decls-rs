// Generated macro for macro_13 (macro)
macro_rules! Depcratemacro_13 {
() => {
// Module: crate
// Provides: {"macro_13"}
// Dependencies: {}
# [cfg (feature = "require-cas")] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (any (not (portable_atomic_no_atomic_cas) , portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,))))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (any (target_has_atomic = "ptr" , portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,))))] compile_error ! ("dependents require atomic CAS but not available on this target by default;\n\
    consider enabling one of the `unsafe-assume-single-core` or `critical-section` Cargo features.\n\
    see <https://docs.rs/portable-atomic/latest/portable_atomic/#optional-features> for more.") ;
};
}
