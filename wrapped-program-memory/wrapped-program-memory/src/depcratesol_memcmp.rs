// Generated macro for sol_memcmp (function)
macro_rules! Depcratesol_memcmp {
() => {
// Module: crate
// Provides: {"sol_memcmp"}
// Dependencies: {}
# [doc = " Like C `memcmp`."] # [doc = ""] # [doc = " # Arguments"] # [doc = ""] # [doc = " - `s1` - Slice to be compared"] # [doc = " - `s2` - Slice to be compared"] # [doc = " - `n` - Number of bytes to compare"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " When executed within a SBF program, the memory regions spanning `n` bytes"] # [doc = " from from the start of `dst` and `src` must be mapped program memory. If not,"] # [doc = " the program will abort."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It does not verify that `n` is less than or equal to the lengths of the"] # [doc = " `dst` and `src` slices passed to it &mdash; it will read bytes beyond the"] # [doc = " slices."] # [doc = ""] # [doc = " Specifying an `n` greater than either the length of `dst` or `src` will"] # [doc = " likely introduce undefined behavior."] # [inline] pub unsafe fn sol_memcmp (s1 : & [u8] , s2 : & [u8] , n : usize) -> i32 { let mut result : MaybeUninit < i32 > = MaybeUninit :: uninit () ; # [cfg (any (target_os = "solana" , target_arch = "bpf"))] syscalls :: sol_memcmp_ (s1 . as_ptr () , s2 . as_ptr () , n as u64 , result . as_mut_ptr ()) ; # [cfg (not (any (target_os = "solana" , target_arch = "bpf")))] stubs :: sol_memcmp (s1 . as_ptr () , s2 . as_ptr () , n , result . as_mut_ptr ()) ; result . assume_init () }
};
}
