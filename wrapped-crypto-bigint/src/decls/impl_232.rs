macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < T > ConstantTimeEq for Odd < T > where T : ConstantTimeEq + ? Sized , { fn ct_eq (& self , other : & Self) -> Choice { self . 0 . ct_eq (& other . 0) } }
    };
}

impl_232!();