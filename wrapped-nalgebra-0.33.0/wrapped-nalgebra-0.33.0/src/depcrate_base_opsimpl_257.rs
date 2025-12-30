// Generated macro for impl_257 (impl)
macro_rules! Depcrate_base_opsimpl_257 {
() => {
// Module: crate::base::ops
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , T , R : DimName , C : DimName > iter :: Sum < & 'a OMatrix < T , R , C > > for OMatrix < T , R , C > where T : Scalar + ClosedAddAssign + Zero , DefaultAllocator : Allocator < R , C > , { fn sum < I : Iterator < Item = & 'a OMatrix < T , R , C > > > (iter : I) -> OMatrix < T , R , C > { iter . fold (Matrix :: zero () , | acc , x | acc + x) } }
};
}
