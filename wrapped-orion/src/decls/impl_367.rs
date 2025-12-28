macro_rules! deps {
    () => {
        FieldElement!();
    };
}

macro_rules! impl_367 {
    () => {
        deps!();
        impl core :: fmt :: Debug for FieldElement { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (f , "FieldElement({:?})" , & self . 0 . 0) } }
    };
}

impl_367!()