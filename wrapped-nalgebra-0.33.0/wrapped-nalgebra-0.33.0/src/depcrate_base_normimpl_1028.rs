// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_base_normimpl_1028 {
() => {
// Module: crate::base::norm
// Provides: {"impl_1028"}
// Dependencies: {}
impl < T : SimdComplexField , R : Dim , C : Dim > Normed for OMatrix < T , R , C > where DefaultAllocator : Allocator < R , C > , { type Norm = T :: SimdRealField ; # [inline] fn norm (& self) -> T :: SimdRealField { self . norm () } # [inline] fn norm_squared (& self) -> T :: SimdRealField { self . norm_squared () } # [inline] fn scale_mut (& mut self , n : Self :: Norm) { self . scale_mut (n) } # [inline] fn unscale_mut (& mut self , n : Self :: Norm) { self . unscale_mut (n) } }
};
}
