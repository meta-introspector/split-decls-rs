// Generated macro for sol_memset (function)
macro_rules! Depcratesol_memset {
() => {
// Module: crate
// Provides: {"sol_memset"}
// Dependencies: {}
# [doc = " Like C `memset`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " - `s` - Slice to be set"] # [doc = " - `c` - Repeated byte to set"] # [doc = " - `n` - Number of bytes to set"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When executed within a SBF program, the memory region spanning `n` bytes"] # [doc = " from from the start of `s` must be mapped program memory. If not, the program"] # [doc = " will abort."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function does not verify that `n` is less than or equal to the length"] # [doc = " of the `s` slice passed to it &mdash; it will write bytes beyond the"] # [doc = " slice."] # [doc = ""] # [doc = " Specifying an `n` greater than the length of `s` will likely introduce"] # [doc = " undefined behavior."] # [inline] pub unsafe fn sol_memset (s : & mut [u8] , c : u8 , n : usize) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] syscalls :: sol_memset_ (s . as_mut_ptr () , c , n as u64) ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] stubs :: sol_memset (s . as_mut_ptr () , c , n) ; }
};
}
