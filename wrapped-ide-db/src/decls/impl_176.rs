macro_rules! deps {
    () => {
        ChangeAnnotationId!();
        Result!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl fmt :: Display for ChangeAnnotationId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } }
    };
}

impl_176!()