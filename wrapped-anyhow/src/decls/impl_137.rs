macro_rules! deps {
    () => {
        Result!();
        MessageError!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < M > Debug for MessageError < M > where M : Display + Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Debug :: fmt (& self . 0 , f) } }
    };
}

impl_137!();