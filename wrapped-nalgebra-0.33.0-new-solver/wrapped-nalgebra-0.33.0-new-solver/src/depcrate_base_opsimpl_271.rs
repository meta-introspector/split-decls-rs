// Generated macro for impl_271 (impl)
macro_rules! Depcrate_base_opsimpl_271 {
() => {
// Module: crate::base::ops
// Provides: {"impl_271"}
// Dependencies: {}
impl < T , D : DimName > iter :: Product for OMatrix < T , D , D > where T : Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign , DefaultAllocator : Allocator < D , D > , { fn product < I : Iterator < Item = OMatrix < T , D , D > > > (iter : I) -> OMatrix < T , D , D > { iter . fold (Matrix :: one () , | acc , x | acc * x) } }
};
}
