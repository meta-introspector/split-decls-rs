macro_rules! deps {
    () => {
        Limb!();
        NonZero!();
    };
}

macro_rules! RemLimb {
    () => {
        deps!();
        # [doc = " Support for optimized division by a single limb."] pub trait RemLimb : Sized { # [doc = " Computes `self % rhs` using a pre-made reciprocal."] fn rem_limb (& self , rhs : NonZero < Limb >) -> Limb { self . rem_limb_with_reciprocal (& Reciprocal :: new (rhs)) } # [doc = " Computes `self % rhs`."] fn rem_limb_with_reciprocal (& self , reciprocal : & Reciprocal) -> Limb ; }
    };
}

RemLimb!()