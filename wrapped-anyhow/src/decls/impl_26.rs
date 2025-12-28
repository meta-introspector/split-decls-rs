macro_rules! deps {
    () => {
        Result!();
        ContextError!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < C , E > Display for ContextError < C , E > where C : Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . context , f) } }
    };
}

impl_26!()