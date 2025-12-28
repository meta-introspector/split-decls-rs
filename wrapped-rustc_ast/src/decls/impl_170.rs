macro_rules! deps {
    () => {
        ImplPolarity!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl fmt :: Debug for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ImplPolarity :: Positive => "positive" . fmt (f) , ImplPolarity :: Negative (_) => "negative" . fmt (f) , } } }
    };
}

impl_170!()