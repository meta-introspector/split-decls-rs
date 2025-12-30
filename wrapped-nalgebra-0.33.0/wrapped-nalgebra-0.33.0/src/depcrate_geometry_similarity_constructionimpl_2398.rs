// Generated macro for impl_2398 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2398 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2398"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > One for Similarity < T , R , D > where T :: Element : SimdRealField , R : AbstractRotation < T , D > , { # [doc = " Creates a new identity similarity."] # [inline] fn one () -> Self { Self :: identity () } }
};
}
