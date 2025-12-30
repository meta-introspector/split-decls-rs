// Generated macro for impl_265 (impl)
macro_rules! Depcrate_base_opsimpl_265 {
() => {
// Module: crate::base::ops
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'a , T , R1 : Dim , C1 : Dim , R2 : Dim , C2 : Dim , SA , SB > Mul < Matrix < T , R2 , C2 , SB > > for & 'a Matrix < T , R1 , C1 , SA > where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , SB : Storage < T , R2 , C2 > , SA : Storage < T , R1 , C1 > , DefaultAllocator : Allocator < R1 , C2 > , ShapeConstraint : AreMultipliable < R1 , C1 , R2 , C2 > , { type Output = OMatrix < T , R1 , C2 > ; # [inline] fn mul (self , rhs : Matrix < T , R2 , C2 , SB >) -> Self :: Output { self * & rhs } }
};
}
