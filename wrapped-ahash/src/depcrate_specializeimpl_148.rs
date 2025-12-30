// Generated macro for impl_148 (impl)
macro_rules! Depcrate_specializeimpl_148 {
() => {
// Module: crate::specialize
// Provides: {"impl_148"}
// Dependencies: {}
# [cfg (specialize)] impl CallHasher for Vec < u8 > { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
};
}
