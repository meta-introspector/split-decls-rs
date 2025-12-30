// Generated macro for impl_2423 (impl)
macro_rules! Depcrate_geometry_similarity_conversionimpl_2423 {
() => {
// Module: crate::geometry::similarity_conversion
// Provides: {"impl_2423"}
// Dependencies: {}
impl < T : Scalar + Zero + PrimitiveSimdValue , R , const D : usize > From < [Similarity < T :: Element , R :: Element , D > ; 16] > for Similarity < T , R , D > where T : From < [< T as SimdValue > :: Element ; 16] > , R : SimdValue + AbstractRotation < T , D > + From < [< R as SimdValue > :: Element ; 16] > , R :: Element : AbstractRotation < T :: Element , D > , T :: Element : Scalar + Zero + Copy , R :: Element : Scalar + Zero + Copy , { # [inline] fn from (arr : [Similarity < T :: Element , R :: Element , D > ; 16]) -> Self { let iso = Isometry :: from ([arr [0] . isometry , arr [1] . isometry , arr [2] . isometry , arr [3] . isometry , arr [4] . isometry , arr [5] . isometry , arr [6] . isometry , arr [7] . isometry , arr [8] . isometry , arr [9] . isometry , arr [10] . isometry , arr [11] . isometry , arr [12] . isometry , arr [13] . isometry , arr [14] . isometry , arr [15] . isometry ,]) ; let scale = T :: from ([arr [0] . scaling () , arr [1] . scaling () , arr [2] . scaling () , arr [3] . scaling () , arr [4] . scaling () , arr [5] . scaling () , arr [6] . scaling () , arr [7] . scaling () , arr [8] . scaling () , arr [9] . scaling () , arr [10] . scaling () , arr [11] . scaling () , arr [12] . scaling () , arr [13] . scaling () , arr [14] . scaling () , arr [15] . scaling () ,]) ; Self :: from_isometry (iso , scale) } }
};
}
