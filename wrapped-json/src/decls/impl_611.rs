macro_rules! deps {
    () => {
        Result!();
        RawValue!();
        Formatter!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl Display for RawValue { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . write_str (& self . json) } }
    };
}

impl_611!()