// Generated macro for impl_159 (impl)
macro_rules! Depcrate_reducedimpl_159 {
() => {
// Module: crate::reduced
// Provides: {"impl_159"}
// Dependencies: {}
impl Reducer < u128 > for Vanilla < u128 > { impl_uprim_vanilla_core ! (u128) ; # [inline] fn mul (& self , lhs : & u128 , rhs : & u128) -> u128 { udouble :: widening_mul (* lhs , * rhs) % self . 0 } # [inline] fn sqr (& self , target : u128) -> u128 { udouble :: widening_square (target) % self . 0 } }
};
}
