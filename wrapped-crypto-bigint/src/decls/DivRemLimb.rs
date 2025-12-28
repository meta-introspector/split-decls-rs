macro_rules! deps {
    () => {
        Limb!();
        NonZero!();
    };
}

macro_rules! DivRemLimb {
    () => {
        deps!();
        # [doc = " Support for optimized division by a single limb."] pub trait DivRemLimb : Sized { # [doc = " Computes `self / rhs` using a pre-made reciprocal,"] # [doc = " returns the quotient (q) and remainder (r)."] fn div_rem_limb (& self , rhs : NonZero < Limb >) -> (Self , Limb) { self . div_rem_limb_with_reciprocal (& Reciprocal :: new (rhs)) } # [doc = " Computes `self / rhs`, returns the quotient (q) and remainder (r)."] fn div_rem_limb_with_reciprocal (& self , reciprocal : & Reciprocal) -> (Self , Limb) ; }
    };
}

DivRemLimb!();