macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl fmt :: Debug for Label { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "label({:?})" , self . ident) } }
    };
}

impl_2!()