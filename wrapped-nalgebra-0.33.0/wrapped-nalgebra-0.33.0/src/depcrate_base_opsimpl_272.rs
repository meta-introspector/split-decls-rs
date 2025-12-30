// Generated macro for impl_272 (impl)
macro_rules! Depcrate_base_opsimpl_272 {
() => {
// Module: crate::base::ops
// Provides: {"impl_272"}
// Dependencies: {}
impl < 'a , T , D : DimName > iter :: Product < & 'a OMatrix < T , D , D > > for OMatrix < T , D , D > where T : Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign , DefaultAllocator : Allocator < D , D > , { fn product < I : Iterator < Item = & 'a OMatrix < T , D , D > > > (iter : I) -> OMatrix < T , D , D > { iter . fold (Matrix :: one () , | acc , x | acc * x) } }
};
}
