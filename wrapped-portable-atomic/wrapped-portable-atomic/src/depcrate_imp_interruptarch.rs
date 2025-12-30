// Generated macro for arch (module)
macro_rules! Depcrate_imp_interruptarch {
() => {
// Module: crate::imp::interrupt
// Provides: {"arch"}
// Dependencies: {}
# [cfg (not (feature = "critical-section"))] # [cfg_attr (all (target_arch = "arm" , any (target_feature = "mclass" , portable_atomic_target_feature = "mclass") ,) , path = "armv6m.rs")] # [cfg_attr (all (target_arch = "arm" , not (any (target_feature = "mclass" , portable_atomic_target_feature = "mclass")) ,) , path = "armv4t.rs")] # [cfg_attr (target_arch = "avr" , path = "avr.rs")] # [cfg_attr (target_arch = "msp430" , path = "msp430.rs")] # [cfg_attr (any (target_arch = "riscv32" , target_arch = "riscv64") , path = "riscv.rs")] # [cfg_attr (target_arch = "xtensa" , path = "xtensa.rs")] mod arch ;
};
}
