macro_rules! deps {
    () => {
        Result!();
        SectionIndex!();
    };
}

macro_rules! impl_901 {
    () => {
        deps!();
        impl fmt :: Display for SectionIndex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . 0 . fmt (f) } }
    };
}

impl_901!()