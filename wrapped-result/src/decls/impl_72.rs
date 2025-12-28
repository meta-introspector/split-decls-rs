macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Error { fn fmt (& self , fmt : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { let mut debug = fmt . debug_struct ("Error") ; debug . field ("code" , & self . code ()) . field ("message" , & self . message ()) . finish () } }
    };
}

impl_72!();