macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! SquareMod {
    () => {
        deps!();
        # [doc = " Compute `self * self mod p`."] pub trait SquareMod < Mod = NonZero < Self > > { # [doc = " Output type."] type Output ; # [doc = " Compute `self * self mod p`."] fn square_mod (& self , p : & Mod) -> Self :: Output ; }
    };
}

SquareMod!();