// Generated macro for impl_2953 (impl)
macro_rules! Depcrate_linalg_svdimpl_2953 {
() => {
// Module: crate::linalg::svd
// Provides: {"impl_2953"}
// Dependencies: {}
impl < T : ComplexField , R : DimMin < C > , C : Dim , S : Storage < T , R , C > > Matrix < T , R , C , S > where DimMinimum < R , C > : DimSub < U1 > , DefaultAllocator : Allocator < R , C > + Allocator < C > + Allocator < R > + Allocator < DimDiff < DimMinimum < R , C > , U1 > > + Allocator < DimMinimum < R , C > , C > + Allocator < R , DimMinimum < R , C > > + Allocator < DimMinimum < R , C > > , { # [doc = " Computes the singular values of this matrix."] # [doc = " The singular values are guaranteed to be sorted in descending order."] # [doc = " If this order is not required consider using `singular_values_unordered`."] # [must_use] pub fn singular_values (& self) -> OVector < T :: RealField , DimMinimum < R , C > > { SVD :: new (self . clone_owned () , false , false) . singular_values } }
};
}
