// Generated macro for impl_2159 (impl)
macro_rules! Depcrate_geometry_scale_constructionimpl_2159 {
() => {
// Module: crate::geometry::scale_construction
// Provides: {"impl_2159"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T : Scalar + Arbitrary + Send , const D : usize > Arbitrary for Scale < T , D > where Owned < T , crate :: Const < D > > : Send , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { let v : SVector < T , D > = Arbitrary :: arbitrary (rng) ; Self :: from (v) } }
};
}
