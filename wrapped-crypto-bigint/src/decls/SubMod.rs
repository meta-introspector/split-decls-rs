macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! SubMod {
    () => {
        deps!();
        # [doc = " Compute `self - rhs mod p`."] pub trait SubMod < Rhs = Self , Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self - rhs mod p`."] # [doc = ""] # [doc = " Assumes `self` and `rhs` are `< p`."] fn sub_mod (& self , rhs : & Rhs , p : & Mod) -> Self :: Output ; }
    };
}

SubMod!()