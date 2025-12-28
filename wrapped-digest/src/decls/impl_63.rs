macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T : OutputSizeUser > PartialEq for CtOutput < T > { # [inline (always)] fn eq (& self , x : & CtOutput < T >) -> bool { self . ct_eq (x) . into () } }
    };
}

impl_63!();