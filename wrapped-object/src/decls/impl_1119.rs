macro_rules! deps {
    () => {
        Result!();
        SectionId!();
    };
}

macro_rules! impl_1119 {
    () => {
        deps!();
        impl fmt :: Debug for SectionId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "SectionId({})" , self . 0) } }
    };
}

impl_1119!();