// Generated macro for instrument (macro)
macro_rules! Depcrateinstrument {
() => {
// Module: crate
// Provides: {"instrument"}
// Dependencies: {}
# [cfg (all (feature = "rust_global_allocator" , not (feature = "benchmark_memory")))] # [macro_export] macro_rules ! instrument { () => { use $ crate :: dlmalloc :: GlobalDlmalloc ; # [global_allocator] static ALLOCATOR : GlobalDlmalloc = GlobalDlmalloc ; const _ : () = { # [no_mangle] fn main (_argc : isize , _argv : * const * const u8) -> isize { self :: main () ; 0 } } ; } ; }
};
}
