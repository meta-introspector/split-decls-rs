macro_rules! deps {
    () => {
        RevwalkWithHideCb!();
        Revwalk!();
        Note!();
        Error!();
        Oid!();
    };
}

macro_rules! impl_712 {
    () => {
        deps!();
        impl < 'repo , 'cb , C : FnMut (Oid) -> bool > RevwalkWithHideCb < 'repo , 'cb , C > { # [doc = " Consumes the `RevwalkWithHideCb` and returns the contained `Revwalk`."] # [doc = ""] # [doc = " Note that this will reset the `Revwalk`."] pub fn into_inner (mut self) -> Result < Revwalk < 'repo > , Error > { self . revwalk . reset () ? ; Ok (self . revwalk) } }
    };
}

impl_712!()