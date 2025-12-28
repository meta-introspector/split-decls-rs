macro_rules! deps {
    () => {
        Wrapping!();
    };
}

macro_rules! impl_426 {
    () => {
        deps!();
        impl < T : ConstantTimeEq > ConstantTimeEq for Wrapping < T > { # [inline] fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
    };
}

impl_426!();