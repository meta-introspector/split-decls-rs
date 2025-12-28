macro_rules! deps {
    () => {
        FnFmt!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        impl < 'a , T , F > fmt :: Display for FnFmt < 'a , T , F > where F : Fn (& 'a T , & mut fmt :: Formatter < '_ >) -> fmt :: Result , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { self . 1 (self . 0 , f) } }
    };
}

impl_576!()