// Generated macro for impl_64 (impl)
macro_rules! Depcrate_wnafimpl_64 {
() => {
// Module: crate::wnaf
// Provides: {"impl_64"}
// Dependencies: {}
impl < B , S : AsRef < [i64] > > Wnaf < usize , B , S > { # [doc = " Performs exponentiation given a base."] pub fn base < G : Group > (& mut self , base : G) -> G where B : AsMut < Vec < G > > , { wnaf_table (self . base . as_mut () , base , self . window_size) ; wnaf_exp (self . base . as_mut () , self . scalar . as_ref ()) } }
};
}
