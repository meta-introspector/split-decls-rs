macro_rules! deps {
    () => {
        TargetLoadError!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl fmt :: Display for TargetLoadError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_68!();