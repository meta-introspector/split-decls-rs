macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        # [doc = " Prints token in a form convenient for debugging."] impl Debug for TokenStream { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . inner , f) } }
    };
}

impl_28!()