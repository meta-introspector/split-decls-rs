// Generated macro for impl_2420 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2420 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2420"}
// Dependencies: {}
impl < T : Scalar + Zero + PrimitiveSimdValue , R , const D : usize > From < [Similarity < T :: Element , R :: Element , D > ; 2] > for Similarity < T , R , D > where T : From < [< T as SimdValue > :: Element ; 2] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 2] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Zero + Copy , R :: Element : Scalar + Zero + Copy , { # [inline] fn from (arr : [Similarity < T :: Element , R :: Element , D > ; 2]) -> Self { let iso = Isometry :: from ([arr [0] . isometry , arr [1] . isometry]) ; let scale = T :: from ([arr [0] . scaling () , arr [1] . scaling ()]) ; Self :: from_isometry (iso , scale) } }
};
}
