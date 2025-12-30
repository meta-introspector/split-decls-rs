// Generated macro for impl_150 (impl)
macro_rules! Depcrate_specializeimpl_150 {
() => {
// Module: crate::specialize
// Provides: {"impl_150"}
// Dependencies: {}
# [cfg (all (specialize))] impl CallHasher for String { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
};
}
