macro_rules! deps {
    () => {
        InvertMod!();
    };
}

macro_rules! InvMod {
    () => {
        deps!();
        # [doc = " Compute `1 / self mod p`."] # [deprecated (since = "0.7.0" , note = "please use `InvertMod` instead")] pub trait InvMod < Rhs = Self > : Sized { # [doc = " Output type."] type Output ; # [doc = " Compute `1 / self mod p`."] fn inv_mod (& self , p : & Rhs) -> CtOption < Self :: Output > ; }
    };
}

InvMod!();