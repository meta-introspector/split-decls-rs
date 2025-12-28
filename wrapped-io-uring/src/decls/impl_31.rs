macro_rules! deps {
    () => {
        Entry32!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Debug for Entry32 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry32") . field ("result" , & self . result ()) . field ("user_data" , & self . user_data ()) . field ("flags" , & self . flags ()) . field ("big_cqe" , & self . big_cqe ()) . finish () } }
    };
}

impl_31!();