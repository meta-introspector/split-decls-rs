// Generated macro for impl_2421 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2421 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2421"}
// Dependencies: {}
impl < T : Scalar + Zero + PrimitiveSimdValue , R , const D : usize > From < [Similarity < T :: Element , R :: Element , D > ; 4] > for Similarity < T , R , D > where T : From < [< T as SimdValue > :: Element ; 4] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 4] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Zero + Copy , R :: Element : Scalar + Zero + Copy , { # [inline] fn from (arr : [Similarity < T :: Element , R :: Element , D > ; 4]) -> Self { let iso = Isometry :: from ([arr [0] . isometry , arr [1] . isometry , arr [2] . isometry , arr [3] . isometry ,]) ; let scale = T :: from ([arr [0] . scaling () , arr [1] . scaling () , arr [2] . scaling () , arr [3] . scaling () ,]) ; Self :: from_isometry (iso , scale) } }
};
}
