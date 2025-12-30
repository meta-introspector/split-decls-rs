// Generated macro for impl_2952 (impl)
macro_rules! Depcrate_linalg_svdimpl_2952 {
() => {
// Module: crate::linalg::svd
// Provides: {"impl_2952"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > where DimMinimum < R , C > : DimSub < U1 > , DefaultAllocator : Allocator < R , C > + Allocator < C > + Allocator < R > + Allocator < DimDiff < DimMinimum < R , C > , U1 > > + Allocator < DimMinimum < R , C > , C > + Allocator < R , DimMinimum < R , C > > + Allocator < DimMinimum < R , C > > + Allocator < DimMinimum < R , C > > + Allocator < DimDiff < DimMinimum < R , C > , U1 > > , { # [doc = " Computes the singular values of this matrix."] # [doc = " The singular values are not guaranteed to be sorted in any particular order."] # [doc = " If a descending order is required, consider using `singular_values` instead."] # [must_use] pub fn singular_values_unordered (& self) -> OVector < T :: RealField , DimMinimum < R , C > > { SVD :: new_unordered (self . clone_owned () , false , false) . singular_values } # [doc = " Computes the rank of this matrix."] # [doc = ""] # [doc = " All singular values below `eps` are considered equal to 0."] # [must_use] pub fn rank (& self , eps : T :: RealField) -> usize { let svd = SVD :: new_unordered (self . clone_owned () , false , false) ; svd . rank (eps) } # [doc = " Computes the pseudo-inverse of this matrix."] # [doc = ""] # [doc = " All singular values below `eps` are considered equal to 0."] pub fn pseudo_inverse (self , eps : T :: RealField) -> Result < OMatrix < T , C , R > , & 'static str > where DefaultAllocator : Allocator < C , R > , { SVD :: new_unordered (self . clone_owned () , true , true) . pseudo_inverse (eps) } }
};
}
