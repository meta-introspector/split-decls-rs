// Generated macro for impl_147 (impl)
macro_rules! Depcrate_specializeimpl_147 {
() => {
// Module: crate::specialize
// Provides: {"impl_147"}
// Dependencies: {}
# [cfg (specialize)] impl CallHasher for [u8] { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_str (value) } }
};
}
