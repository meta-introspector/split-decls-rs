// Generated macro for impl_120 (impl)
macro_rules! Depcrate_specializeimpl_120 {
() => {
// Module: crate::specialize
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (specialize)] impl < T > CallHasher for T where T : Hash + ? Sized , { # [inline] default fn get_hash < H : Hash + ? Sized > (value : & H , random_state : & RandomState) -> u64 { let mut hasher = random_state . build_hasher () ; value . hash (& mut hasher) ; hasher . finish () } }
};
}
