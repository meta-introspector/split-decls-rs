macro_rules! deps {
    () => {
        InvertMod!();
        InvMod!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        # [allow (deprecated)] impl < T , Rhs > InvMod < Rhs > for T where T : InvertMod < Rhs > , { type Output = < T as InvertMod < Rhs > > :: Output ; fn inv_mod (& self , p : & Rhs) -> CtOption < Self :: Output > { self . invert_mod (p) } }
    };
}

impl_290!()