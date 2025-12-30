// Generated macro for macro_110 (macro)
macro_rules! Depcratemacro_110 {
() => {
// Module: crate
// Provides: {"macro_110"}
// Dependencies: {}
cfg_if ! { if # [cfg (chacha20_force_soft)] { type Tokens = () ; } else if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { cfg_if ! { if # [cfg (chacha20_force_avx2)] { # [cfg (not (target_feature = "avx2"))] compile_error ! ("You must enable `avx2` target feature with \
                    `chacha20_force_avx2` configuration option") ; type Tokens = () ; } else if # [cfg (chacha20_force_sse2)] { # [cfg (not (target_feature = "sse2"))] compile_error ! ("You must enable `sse2` target feature with \
                    `chacha20_force_sse2` configuration option") ; type Tokens = () ; } else { cpufeatures :: new ! (avx2_cpuid , "avx2") ; cpufeatures :: new ! (sse2_cpuid , "sse2") ; type Tokens = (avx2_cpuid :: InitToken , sse2_cpuid :: InitToken) ; } } } else { type Tokens = () ; } }
};
}
