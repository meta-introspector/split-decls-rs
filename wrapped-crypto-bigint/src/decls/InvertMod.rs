macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! InvertMod {
    () => {
        deps!();
        # [doc = " Compute `1 / self mod p`."] pub trait InvertMod < Mod = NonZero < Self > > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute `1 / self mod p`."] fn invert_mod (& self , p : & Mod) -> CtOption < Self :: Output > ; }
    };
}

InvertMod!()