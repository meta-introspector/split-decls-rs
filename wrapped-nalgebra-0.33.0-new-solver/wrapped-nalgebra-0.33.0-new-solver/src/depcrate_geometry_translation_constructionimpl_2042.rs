// Generated macro for impl_2042 (impl)
macro_rules! Depcrate_geometry_translation_constructionimpl_2042 {
() => {
// Module: crate::geometry::translation_construction
// Provides: {"impl_2042"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : Scalar + Arbitrary + Send , const D : usize > Arbitrary for Translation < T , D > where Owned < T , crate :: Const < D > > : Send , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { let v : SVector < T , D > = Arbitrary :: arbitrary (rng) ; Self :: from (v) } }
};
}
