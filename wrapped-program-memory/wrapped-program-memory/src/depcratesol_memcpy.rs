// Generated macro for sol_memcpy (function)
macro_rules! Depcratesol_memcpy {
() => {
// Module: crate
// Provides: {"sol_memcpy"}
// Dependencies: {}
# [doc = " Like C `memcpy`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " - `dst` - Destination"] # [doc = " - `src` - Source"] # [doc = " - `n` - Number of bytes to copy"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When executed within a SBF program, the memory regions spanning `n` bytes"] # [doc = " from from the start of `dst` and `src` must be mapped program memory. If not,"] # [doc = " the program will abort."] # [doc = ""] # [doc = " The memory regions spanning `n` bytes from `dst` and `src` from the start"] # [doc = " of `dst` and `src` must not overlap. If they do, then the program will abort"] # [doc = " or, if run outside of the SBF VM, will panic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This function does not verify that `n` is less than or equal to the"] # [doc = " lengths of the `dst` and `src` slices passed to it &mdash; it will copy"] # [doc = " bytes to and from beyond the slices."] # [doc = ""] # [doc = " Specifying an `n` greater than either the length of `dst` or `src` will"] # [doc = " likely introduce undefined behavior."] # [inline] pub unsafe fn sol_memcpy (dst : & mut [u8] , src : & [u8] , n : usize) { # [cfg (any (target_os = "solana" , target_arch = "bpf"))] syscalls :: sol_memcpy_ (dst . as_mut_ptr () , src . as_ptr () , n as u64) ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] stubs :: sol_memcpy (dst . as_mut_ptr () , src . as_ptr () , n) ; }
};
}
