// Generated macro for impl_250 (impl)
macro_rules! Depcrate_base_opsimpl_250 {
() => {
// Module: crate::base::ops
// Provides: {"impl_250"}
// Dependencies: {}
impl < 'a , T , R : Dim , C : Dim , S > Neg for & 'a Matrix < T , R , C , S > where T : Scalar + ClosedNeg , S : Storage < T , R , C > , DefaultAllocator : Allocator < R , C > , { type Output = OMatrix < T , R , C > ; # [inline] fn neg (self) -> Self :: Output { - self . clone_owned () } }
};
}
