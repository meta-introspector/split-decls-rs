// Generated macro for sys_rand (function)
macro_rules! Depcrate_syscalls_entropysys_rand {
() => {
// Module: crate::syscalls::entropy
// Provides: {"sys_rand"}
// Dependencies: {}
# [doc = " The function computes a sequence of pseudo-random integers"] # [doc = " in the range of 0 to RAND_MAX"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_rand () -> u32 { generate_park_miller_lehmer_random_number () }
};
}
