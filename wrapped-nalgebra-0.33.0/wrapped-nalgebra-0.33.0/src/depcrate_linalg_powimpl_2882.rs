// Generated macro for impl_2882 (impl)
macro_rules! Depcrate_linalg_powimpl_2882 {
() => {
// Module: crate::linalg::pow
// Provides: {"impl_2882"}
// Dependencies: {}
impl < T , D , S > Matrix < T , D , D , S > where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , D : DimMin < D , Output = D > , S : StorageMut < T , D , D > , DefaultAllocator : Allocator < D , D > + Allocator < D > , { # [doc = " Raises this matrix to an integral power `exp` in-place."] pub fn pow_mut (& mut self , mut exp : u32) { if exp == 0 { self . fill_with_identity () ; } else if exp > 1 { let mut x = self . clone_owned () ; let mut workspace = self . clone_owned () ; if exp % 2 == 0 { self . fill_with_identity () ; } else { exp -= 1 ; } loop { if exp % 2 == 1 { self . mul_to (& x , & mut workspace) ; self . copy_from (& workspace) ; } exp /= 2 ; if exp == 0 { break ; } x . mul_to (& x , & mut workspace) ; x . copy_from (& workspace) ; } } } }
};
}
