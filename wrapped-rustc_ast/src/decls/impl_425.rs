macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl fmt :: Debug for LazyAttrTokenStream { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "LazyAttrTokenStream({:?})" , self . to_attr_token_stream ()) } }
    };
}

impl_425!();