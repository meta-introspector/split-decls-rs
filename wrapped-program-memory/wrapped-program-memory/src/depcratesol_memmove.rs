// Generated macro for sol_memmove (function)
macro_rules! Depcratesol_memmove {
() => {
// Module: crate
// Provides: {"sol_memmove"}
// Dependencies: {}
# [doc = " Like C `memmove`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " - `dst` - Destination"] # [doc = " - `src` - Source"] # [doc = " - `n` - Number of bytes to copy"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When executed within a SBF program, the memory regions spanning `n` bytes"] # [doc = " from from `dst` and `src` must be mapped program memory. If not, the program"] # [doc = " will abort."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The same safety rules apply as in [`ptr::copy`]."] # [doc = ""] # [doc = " [`ptr::copy`]: https://doc.rust-lang.org/core/ptr/fn.copy.html"] # [inline] pub unsafe fn sol_memmove (dst : * mut u8 , src : * const u8 , n : usize) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] syscalls :: sol_memmove_ (dst , src , n as u64) ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] stubs :: sol_memmove (dst , src , n) ; }
};
}
