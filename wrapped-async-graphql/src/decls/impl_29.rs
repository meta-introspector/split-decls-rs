macro_rules! deps {
    () => {
        ServerError!();
        Result!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Debug for ServerError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . debug_struct ("ServerError") . field ("message" , & self . message) . field ("locations" , & self . locations) . field ("path" , & self . path) . field ("extensions" , & self . extensions) . finish () } }
    };
}

impl_29!();