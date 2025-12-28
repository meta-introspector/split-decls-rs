macro_rules! deps {
    () => {
        GenericArray!();
        ArrayLength!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T : PartialEq , N : ArrayLength > PartialEq for GenericArray < T , N > { # [inline (always)] fn eq (& self , other : & Self) -> bool { * * self == * * other } }
    };
}

impl_27!();