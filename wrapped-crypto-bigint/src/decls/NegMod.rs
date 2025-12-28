macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! NegMod {
    () => {
        deps!();
        # [doc = " Compute `-self mod p`."] pub trait NegMod < Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `-self mod p`."] # [must_use] fn neg_mod (& self , p : & Mod) -> Self :: Output ; }
    };
}

NegMod!()