// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_base_normimpl_1029 {
() => {
// Module: crate::base::norm
// Provides: {"impl_1029"}
// Dependencies: {}
impl < T : Scalar + ClosedNeg , R : Dim , C : Dim > Neg for Unit < OMatrix < T , R , C > > where DefaultAllocator : Allocator < R , C > , { type Output = Unit < OMatrix < T , R , C > > ; # [inline] fn neg (self) -> Self :: Output { Unit :: new_unchecked (- self . value) } }
};
}
