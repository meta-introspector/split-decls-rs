// Generated macro for impl_1042 (impl)
macro_rules! Depcrate_base_propertiesimpl_1042 {
() => {
// Module: crate::base::properties
// Provides: {"impl_1042"}
// Dependencies: {}
impl < T : RealField , D : Dim , S : Storage < T , D , D > > SquareMatrix < T , D , S > where DefaultAllocator : Allocator < D , D > , { # [doc = " Checks that this matrix is orthogonal and has a determinant equal to 1."] # [inline] # [must_use] pub fn is_special_orthogonal (& self , eps : T) -> bool where D : DimMin < D , Output = D > , DefaultAllocator : Allocator < D > , { self . is_square () && self . is_orthogonal (eps) && self . determinant () > T :: zero () } # [doc = " Returns `true` if this matrix is invertible."] # [inline] # [must_use] pub fn is_invertible (& self) -> bool { self . clone_owned () . try_inverse () . is_some () } }
};
}
