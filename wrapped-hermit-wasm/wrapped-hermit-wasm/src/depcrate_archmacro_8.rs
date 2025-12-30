// Generated macro for macro_8 (macro)
macro_rules! Depcrate_archmacro_8 {
() => {
// Module: crate::arch
// Provides: {"macro_8"}
// Dependencies: {}
cfg_if :: cfg_if ! { if # [cfg (target_arch = "aarch64")] { pub (crate) mod aarch64 ; } else if # [cfg (target_arch = "x86_64")] { pub (crate) mod x86_64 ; } else if # [cfg (target_arch = "riscv64")] { pub (crate) mod riscv64 ; } }
};
}
