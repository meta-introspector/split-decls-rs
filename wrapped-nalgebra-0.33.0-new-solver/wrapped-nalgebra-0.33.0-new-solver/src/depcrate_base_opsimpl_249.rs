// Generated macro for impl_249 (impl)
macro_rules! Depcrate_base_opsimpl_249 {
() => {
// Module: crate::base::ops
// Provides: {"impl_249"}
// Dependencies: {}
impl < T , R : Dim , C : Dim , S > Neg for Matrix < T , R , C , S > where T : Scalar + ClosedNeg , S : Storage < T , R , C > , DefaultAllocator : Allocator < R , C > , { type Output = OMatrix < T , R , C > ; # [inline] fn neg (self) -> Self :: Output { let mut res = self . into_owned () ; res . neg_mut () ; res } }
};
}
