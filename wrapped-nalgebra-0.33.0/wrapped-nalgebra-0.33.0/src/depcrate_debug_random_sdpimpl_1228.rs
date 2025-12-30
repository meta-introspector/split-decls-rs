// Generated macro for impl_1228 (impl)
macro_rules! Depcrate_debug_random_sdpimpl_1228 {
() => {
// Module: crate::debug::random_sdp
// Provides: {"impl_1228"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : ComplexField + Arbitrary + Send , D : Dim > Arbitrary for RandomSDP < T , D > where DefaultAllocator : Allocator < D , D > , Owned < T , D , D > : Clone + Send , { fn arbitrary (g : & mut Gen) -> Self { let dim = D :: try_to_usize () . unwrap_or (1 + usize :: arbitrary (g) % 50) ; Self :: new (D :: from_usize (dim) , | | T :: arbitrary (g)) } }
};
}
