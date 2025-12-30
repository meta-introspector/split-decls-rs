// Generated macro for impl_65 (impl)
macro_rules! Depcrate_wnafimpl_65 {
() => {
// Module: crate::wnaf
// Provides: {"impl_65"}
// Dependencies: {}
impl < B , S : AsMut < Vec < i64 > > > Wnaf < usize , B , S > { # [doc = " Performs exponentiation given a scalar."] pub fn scalar < G : Group > (& mut self , scalar : & < G as Group > :: Scalar) -> G where B : AsRef < [G] > , { wnaf_form (self . scalar . as_mut () , scalar . to_repr () , self . window_size) ; wnaf_exp (self . base . as_ref () , self . scalar . as_mut ()) } }
};
}
