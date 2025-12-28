macro_rules! deps {
    () => {
        Result!();
        InvalidReasonPhrase!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl std :: fmt :: Display for InvalidReasonPhrase { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "Invalid byte in reason phrase: {}" , self . bad_byte) } }
    };
}

impl_133!();