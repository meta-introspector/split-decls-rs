macro_rules! deps {
    () => {
        MessageError!();
        Result!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < M > Display for MessageError < M > where M : Display + Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . 0 , f) } }
    };
}

impl_138!()