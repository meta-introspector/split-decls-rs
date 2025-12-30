// Generated macro for atomic_32_macros (module)
macro_rules! Depcrate_cfgsatomic_32_macros {
() => {
// Module: crate::cfgs
// Provides: {"atomic_32_macros"}
// Dependencies: {}
# [cfg (not (all (any (not (target_pointer_width = "16") , feature = "fallback") , not (all (portable_atomic_no_atomic_load_store , not (any (target_arch = "avr" , target_arch = "msp430" , target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" ,)) ,)) ,)))] # [macro_use] mod atomic_32_macros { # [macro_export] macro_rules ! cfg_has_atomic_32 { ($ ($ tt : tt) *) => { } ; } # [macro_export] macro_rules ! cfg_no_atomic_32 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; } }
};
}
