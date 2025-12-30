// Generated macro for impl_119 (impl)
macro_rules! Depcrate_specializeimpl_119 {
() => {
// Module: crate::specialize
// Provides: {"impl_119"}
// Dependencies: {}
# [cfg (not (specialize))] impl < T > CallHasher for T where T : Hash + ? Sized , { # [inline] fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { let mut hasher = random_state . build_hasher () ; value . hash (& mut hasher) ; hasher . finish () } }
};
}
