macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl Debug for Error { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Error") . field ("message" , & self . message) . field ("extensions" , & self . extensions) . finish () } }
    };
}

impl_42!()