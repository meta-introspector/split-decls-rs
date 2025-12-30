// Generated macro for impl_3138 (impl)
macro_rules! Depcrate_third_party_alga_alga_dual_quaternionimpl_3138 {
() => {
// Module: crate::third_party::alga::alga_dual_quaternion
// Provides: {"impl_3138"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField > FiniteDimVectorSpace for DualQuaternion < T > { # [inline] fn dimension () -> usize { 8 } # [inline] fn canonical_basis_element (i : usize) -> Self { if i < 4 { DualQuaternion :: from_real_and_dual (Quaternion :: canonical_basis_element (i) , Quaternion :: zero () ,) } else { DualQuaternion :: from_real_and_dual (Quaternion :: zero () , Quaternion :: canonical_basis_element (i - 4) ,) } } # [inline] fn dot (& self , other : & Self) -> T { self . real . dot (& other . real) + self . dual . dot (& other . dual) } # [inline] unsafe fn component_unchecked (& self , i : usize) -> & T { self . as_ref () . get_unchecked (i) } # [inline] unsafe fn component_unchecked_mut (& mut self , i : usize) -> & mut T { self . as_mut () . get_unchecked_mut (i) } }
};
}
