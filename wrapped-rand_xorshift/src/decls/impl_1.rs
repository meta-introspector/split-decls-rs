macro_rules! deps {
    () => {
        XorShiftRng!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Debug for XorShiftRng { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "XorShiftRng {{}}") } }
    };
}

impl_1!();