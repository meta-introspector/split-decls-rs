// Generated macro for impl_2396 (impl)
macro_rules! Depcrate_geometry_similarity_constructionimpl_2396 {
() => {
// Module: crate::geometry::similarity_construction
// Provides: {"impl_2396"}
// Dependencies: {}
impl < T : SimdRealField , R , const D : usize > Default for Similarity < T , R , D > where T :: Element : SimdRealField , R : AbstractRotation < T , D > , { fn default () -> Self { Self :: identity () } }
};
}
