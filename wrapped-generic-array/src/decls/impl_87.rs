macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < T , N : ArrayLength > ConstantTimeEq for GenericArray < T , N > where T : ConstantTimeEq , { # [inline] fn ct_eq (& self , other : & Self) -> subtle :: Choice { self . as_slice () . ct_eq (other . as_slice ()) } }
    };
}

impl_87!();