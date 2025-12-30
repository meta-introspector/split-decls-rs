// Generated macro for impl_2883 (impl)
macro_rules! Depcrate_linalg_powimpl_2883 {
() => {
// Module: crate::linalg::pow
// Provides: {"impl_2883"}
// Dependencies: {}
impl < T , D , S : Storage < T , D , D > > Matrix < T , D , D , S > where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , D : DimMin < D , Output = D > , S : StorageMut < T , D , D > , DefaultAllocator : Allocator < D , D > + Allocator < D > , { # [doc = " Raise this matrix to an integral power `exp`."] # [must_use] pub fn pow (& self , exp : u32) -> OMatrix < T , D , D > { let mut result = self . clone_owned () ; result . pow_mut (exp) ; result } }
};
}
