// Generated macro for impl_255 (impl)
macro_rules! Depcrate_base_opsimpl_255 {
() => {
// Module: crate::base::ops
// Provides: {"impl_255"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > iter :: Sum for OMatrix < T , R , C > where T : Scalar + ClosedAddAssign + Zero , DefaultAllocator : Allocator < R , C > , { fn sum < I : Iterator < Item = OMatrix < T , R , C > > > (iter : I) -> OMatrix < T , R , C > { iter . fold (Matrix :: zero () , | acc , x | acc + x) } }
};
}
