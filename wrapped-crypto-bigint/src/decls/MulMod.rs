macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! MulMod {
    () => {
        deps!();
        # [doc = " Compute `self * rhs mod p`."] pub trait MulMod < Rhs = Self , Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self * rhs mod p`."] fn mul_mod (& self , rhs : & Rhs , p : & Mod) -> Self :: Output ; }
    };
}

MulMod!();