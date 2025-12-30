// Generated macro for atomic_64_macros (module)
macro_rules! Depcrate_cfgsatomic_64_macros {
() => {
// Module: crate::cfgs
// Provides: {"atomic_64_macros"}
// Dependencies: {}
# [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (any (all (feature = "fallback" , any (not (portable_atomic_no_atomic_cas) , portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,) ,) , not (portable_atomic_no_atomic_64) , not (any (target_pointer_width = "16" , target_pointer_width = "32")) , all (target_arch = "riscv32" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas") ,) ,))))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (any (all (feature = "fallback" , any (target_has_atomic = "ptr" , portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,) ,) , target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) , all (target_arch = "riscv32" , not (any (miri , portable_atomic_sanitize_thread)) , any (not (portable_atomic_no_asm) , portable_atomic_unstable_asm) , any (target_feature = "zacas" , portable_atomic_target_feature = "zacas") ,) ,))))] # [macro_use] mod atomic_64_macros { # [macro_export] macro_rules ! cfg_has_atomic_64 { ($ ($ tt : tt) *) => { } ; } # [macro_export] macro_rules ! cfg_no_atomic_64 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; } }
};
}
