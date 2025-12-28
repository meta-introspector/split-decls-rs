macro_rules! deps {
    () => {
        Result!();
        Formatter!();
        Error!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Error({:?}, line: {}, column: {})" , self . err . code . to_string () , self . err . line , self . err . column) } }
    };
}

impl_67!()