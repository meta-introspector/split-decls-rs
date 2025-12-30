// Generated macro for atomic_8_16_macros (module)
macro_rules! Depcrate_cfgsatomic_8_16_macros {
() => {
// Module: crate::cfgs
// Provides: {"atomic_8_16_macros"}
// Dependencies: {}
# [cfg (all (portable_atomic_no_atomic_load_store , not (any (target_arch = "avr" , target_arch = "msp430" , target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" ,)) ,))] # [macro_use] mod atomic_8_16_macros { # [macro_export] macro_rules ! cfg_has_atomic_8 { ($ ($ tt : tt) *) => { } ; } # [macro_export] macro_rules ! cfg_no_atomic_8 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; } # [macro_export] macro_rules ! cfg_has_atomic_16 { ($ ($ tt : tt) *) => { } ; } # [macro_export] macro_rules ! cfg_no_atomic_16 { ($ ($ tt : tt) *) => { $ ($ tt) * } ; } }
};
}
