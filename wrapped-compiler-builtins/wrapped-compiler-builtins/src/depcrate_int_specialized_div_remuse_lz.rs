// Generated macro for USE_LZ (const)
macro_rules! Depcrate_int_specialized_div_remUSE_LZ {
() => {
// Module: crate::int::specialized_div_rem
// Provides: {"USE_LZ"}
// Dependencies: {}
const USE_LZ : bool = { if cfg ! (target_arch = "arm") { if cfg ! (target_feature = "thumb-mode") { cfg ! (target_feature = "v6t2") } else { cfg ! (target_feature = "v5te") } } else if cfg ! (any (target_arch = "sparc" , target_arch = "sparc64")) { cfg ! (target_feature = "vis3") } else if cfg ! (any (target_arch = "riscv32" , target_arch = "riscv64")) { cfg ! (target_feature = "zbb") } else { true } } ;
};
}
