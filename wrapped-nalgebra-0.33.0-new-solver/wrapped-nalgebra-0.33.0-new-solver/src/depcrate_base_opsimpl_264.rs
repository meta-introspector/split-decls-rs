// Generated macro for impl_264 (impl)
macro_rules! Depcrate_base_opsimpl_264 {
() => {
// Module: crate::base::ops
// Provides: {"impl_264"}
// Dependencies: {}
impl < 'a , 'b , T , R1 : Dim , C1 : Dim , R2 : Dim , C2 : Dim , SA , SB > Mul < & 'b Matrix < T , R2 , C2 , SB > > for & 'a Matrix < T , R1 , C1 , SA > where T : Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign , SA : Storage < T , R1 , C1 > , SB : Storage < T , R2 , C2 > , DefaultAllocator : Allocator < R1 , C2 > , ShapeConstraint : AreMultipliable < R1 , C1 , R2 , C2 > , { type Output = OMatrix < T , R1 , C2 > ; # [inline] fn mul (self , rhs : & 'b Matrix < T , R2 , C2 , SB >) -> Self :: Output { let mut res = Matrix :: uninit (self . shape_generic () . 0 , rhs . shape_generic () . 1) ; unsafe { gemm_uninit (Uninit , & mut res , T :: one () , self , rhs , T :: zero ()) ; res . assume_init () } } }
};
}
