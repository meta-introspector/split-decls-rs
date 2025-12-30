// Generated macro for impl_2401 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2401 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2401"}
// Dependencies: {}
# [cfg (feature = "arbitrary")] impl < T , R , const D : usize > Arbitrary for Similarity < T , R , D > where T : crate :: RealField + Arbitrary + Send , T :: Element : crate :: RealField , R : AbstractRotation < T , D > + Arbitrary + Send , Owned < T , crate :: Const < D > > : Send , { # [inline] fn arbitrary (rng : & mut Gen) -> Self { let mut s : T = Arbitrary :: arbitrary (rng) ; while s . is_zero () { s = Arbitrary :: arbitrary (rng) } Self :: from_isometry (Arbitrary :: arbitrary (rng) , s) } }
};
}
