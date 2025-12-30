// Generated macro for impl_193 (impl)
macro_rules! Depcrateimpl_193 {
() => {
// Module: crate
// Provides: {"impl_193"}
// Dependencies: {}
# [cfg (feature = "malloc_size_of")] impl < T , const N : usize > MallocShallowSizeOf for SmallVec < T , N > { fn shallow_size_of (& self , ops : & mut MallocSizeOfOps) -> usize { if self . spilled () { unsafe { ops . malloc_size_of (self . as_ptr ()) } } else { 0 } } }
};
}
