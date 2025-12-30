// Generated macro for init_entropy (function)
macro_rules! Depcrate_syscalls_entropyinit_entropy {
() => {
// Module: crate::syscalls::entropy
// Provides: {"init_entropy"}
// Dependencies: {}
pub (crate) fn init_entropy () { let seed : u32 = arch :: processor :: get_timestamp () as u32 ; * PARK_MILLER_LEHMER_SEED . lock () = seed ; }
};
}
