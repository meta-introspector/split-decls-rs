// Generated macro for RemLimb (trait)
macro_rules! Depcrate_traitsRemLimb {
() => {
// Module: crate::traits
// Provides: {"RemLimb"}
// Dependencies: {}
# [doc = " Support for optimized division by a single limb."] pub trait RemLimb : Sized { # [doc = " Computes `self % rhs` using a pre-made reciprocal."] fn rem_limb (& self , rhs : NonZero < Limb >) -> Limb { self . rem_limb_with_reciprocal (& Reciprocal :: new (rhs)) } # [doc = " Computes `self % rhs`."] fn rem_limb_with_reciprocal (& self , reciprocal : & Reciprocal) -> Limb ; }
};
}
