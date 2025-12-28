macro_rules! deps {
    () => {
        InternalNumeric!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl fmt :: Debug for InternalNumeric { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "<InternalNumeric>") } }
    };
}

impl_261!()