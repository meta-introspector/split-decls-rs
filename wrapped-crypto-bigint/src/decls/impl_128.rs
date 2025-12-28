macro_rules! deps {
    () => {
        JacobiSymbol!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl ConstantTimeEq for JacobiSymbol { fn ct_eq (& self , other : & Self) -> Choice { (* self as i8) . ct_eq (& (* other as i8)) } }
    };
}

impl_128!();