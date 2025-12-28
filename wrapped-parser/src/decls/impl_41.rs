macro_rules! deps {
    () => {
        Result!();
        Type!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl Display for Type { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { self . base . fmt (f) ? ; if ! self . nullable { f . write_char ('!') ? ; } Ok (()) } }
    };
}

impl_41!()