// Generated macro for sys_srand (function)
macro_rules! Depcrate_syscalls_entropysys_srand {
() => {
// Module: crate::syscalls::entropy
// Provides: {"sys_srand"}
// Dependencies: {}
# [doc = " The function sets its argument as the seed for a new sequence"] # [doc = " of pseudo-random numbers to be returned by rand()"] # [hermit_macro :: system] # [unsafe (no_mangle)] pub extern "C" fn sys_srand (seed : u32) { * (PARK_MILLER_LEHMER_SEED . lock ()) = seed ; }
};
}
