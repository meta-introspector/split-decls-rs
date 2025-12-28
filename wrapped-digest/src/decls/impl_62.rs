macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T : OutputSizeUser > ConstantTimeEq for CtOutput < T > { # [inline (always)] fn ct_eq (& self , other : & Self) -> Choice { self . bytes . ct_eq (& other . bytes) } }
    };
}

impl_62!()