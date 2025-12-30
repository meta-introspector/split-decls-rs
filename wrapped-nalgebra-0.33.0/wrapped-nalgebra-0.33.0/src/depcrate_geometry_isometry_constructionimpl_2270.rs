// Generated macro for impl_2270 (impl)
macro_rules! Depcrate_geometry_isometry_constructionimpl_2270 {
() => {
// Module: crate::geometry::isometry_construction
// Provides: {"impl_2270"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T , R , const D : usize > Arbitrary for Isometry < T , R , D > where T : SimdRealField + Arbitrary + Send , T :: Element : SimdRealField , R : AbstractRotation < T , D > + Arbitrary + Send , Owned < T , crate :: Const < D > > : Send , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { Self :: from_parts (Arbitrary :: arbitrary (rng) , Arbitrary :: arbitrary (rng)) } }
};
}
