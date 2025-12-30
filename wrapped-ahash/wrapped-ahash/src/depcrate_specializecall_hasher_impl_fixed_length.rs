// Generated macro for call_hasher_impl_fixed_length (macro)
macro_rules! Depcrate_specializecall_hasher_impl_fixed_length {
() => {
// Module: crate::specialize
// Provides: {"call_hasher_impl_fixed_length"}
// Dependencies: {}
macro_rules ! call_hasher_impl_fixed_length { ($ typ : ty) => { # [cfg (specialize)] impl CallHasher for $ typ { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_fixed_length (value) } } } ; }
};
}
