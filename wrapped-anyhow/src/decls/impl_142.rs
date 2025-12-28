macro_rules! deps {
    () => {
        DisplayError!();
        Result!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < M > Display for DisplayError < M > where M : Display , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
    };
}

impl_142!()