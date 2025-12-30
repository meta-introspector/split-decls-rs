// Generated macro for impl_2422 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2422 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2422"}
// Dependencies: {}
impl < T : Scalar + Zero + PrimitiveSimdValue , R , const D : usize > From < [Similarity < T :: Element , R :: Element , D > ; 8] > for Similarity < T , R , D > where T : From < [< T as SimdValue > :: Element ; 8] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 8] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Zero + Copy , R :: Element : Scalar + Zero + Copy , { # [inline] fn from (arr : [Similarity < T :: Element , R :: Element , D > ; 8]) -> Self { let iso = Isometry :: from ([arr [0] . isometry , arr [1] . isometry , arr [2] . isometry , arr [3] . isometry , arr [4] . isometry , arr [5] . isometry , arr [6] . isometry , arr [7] . isometry ,]) ; let scale = T :: from ([arr [0] . scaling () , arr [1] . scaling () , arr [2] . scaling () , arr [3] . scaling () , arr [4] . scaling () , arr [5] . scaling () , arr [6] . scaling () , arr [7] . scaling () ,]) ; Self :: from_isometry (iso , scale) } }
};
}
