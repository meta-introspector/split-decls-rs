// Generated macro for impl_268 (impl)
macro_rules! Depcrate_base_opsimpl_268 {
() => {
// Module: crate::base::ops
// Provides: {"impl_268"}
// Dependencies: {}
impl < T , R1 , C1 , R2 , SA , SB > MulAssign < Matrix < T , R2 , C1 , SB > > for Matrix < T , R1 , C1 , SA > where R1 : Dim , C1 : Dim , R2 : Dim , T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , SB : Storage < T , R2 , C1 > , SA : StorageMut < T , R1 , C1 > + IsContiguous + Clone , ShapeConstraint : AreMultipliable < R1 , C1 , R2 , C1 > , DefaultAllocator : Allocator < R1 , C1 , Buffer < T > = SA > , { # [inline] fn mul_assign (& mut self , rhs : Matrix < T , R2 , C1 , SB >) { * self = & * self * rhs } }
};
}
