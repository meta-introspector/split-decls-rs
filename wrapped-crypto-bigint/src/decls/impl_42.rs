macro_rules! deps {
    () => {
        Checked!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < T : ConstantTimeEq > ConstantTimeEq for Checked < T > { # [inline] fn ct_eq (& self , rhs : & Self) -> Choice { self . 0 . ct_eq (& rhs . 0) } }
    };
}

impl_42!()