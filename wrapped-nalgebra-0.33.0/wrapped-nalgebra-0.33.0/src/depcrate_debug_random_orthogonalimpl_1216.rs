// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_debug_random_orthogonalimpl_1216 {
() => {
// Module: crate::debug::random_orthogonal
// Provides: {"impl_1216"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : ComplexField + Arbitrary + Send , D : Dim > Arbitrary for RandomOrthogonal < T , D > where DefaultAllocator : Allocator < D , D > , Owned < T , D , D > : Clone + Send , { fn arbitrary (g : & mut Gen) -> Self { let dim = D :: try_to_usize () . unwrap_or (1 + usize :: arbitrary (g) % 50) ; Self :: new (D :: from_usize (dim) , | | T :: arbitrary (g)) } }
};
}
