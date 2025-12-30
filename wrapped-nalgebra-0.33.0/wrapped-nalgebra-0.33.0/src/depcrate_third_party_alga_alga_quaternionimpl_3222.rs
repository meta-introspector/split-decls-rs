// Generated macro for impl_3222 (impl)
macro_rules! Depcrate_third_party_alga_alga_quaternionimpl_3222 {
() => {
// Module: crate::third_party::alga::alga_quaternion
// Provides: {"impl_3222"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > FiniteDimVectorSpace for Quaternion < T > { # [inline] fn dimension () -> usize { 4 } # [inline] fn canonical_basis_element (i : usize) -> Self { Self :: from (Vector4 :: canonical_basis_element (i)) } # [inline] fn dot (& self , other : & Self) -> T { self . coords . dot (& other . coords) } # [inline] unsafe fn component_unchecked (& self , i : usize) -> & T { self . coords . component_unchecked (i) } # [inline] unsafe fn component_unchecked_mut (& mut self , i : usize) -> & mut T { self . coords . component_unchecked_mut (i) } }
};
}
