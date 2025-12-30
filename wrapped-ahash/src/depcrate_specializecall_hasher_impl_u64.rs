// Generated macro for call_hasher_impl_u64 (macro)
macro_rules! Depcrate_specializecall_hasher_impl_u64 {
() => {
// Module: crate::specialize
// Provides: {"call_hasher_impl_u64"}
// Dependencies: {}
macro_rules ! call_hasher_impl_u64 { ($ typ : ty) => { # [cfg (specialize)] impl CallHasher for $ typ { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { random_state . hash_as_u64 (value) } } } ; }
};
}
