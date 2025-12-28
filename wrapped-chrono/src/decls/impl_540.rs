macro_rules! deps {
    () => {
        FixedOffset!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl fmt :: Display for FixedOffset { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (self , f) } }
    };
}

impl_540!()