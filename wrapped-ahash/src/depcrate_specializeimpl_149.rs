// Generated macro for impl_149 (impl)
macro_rules! Depcrate_specializeimpl_149 {
() => {
// Module: crate::specialize
// Provides: {"impl_149"}
// Dependencies: {}
# [cfg (specialize)] impl CallHasher for str { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
};
}
