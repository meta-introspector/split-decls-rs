macro_rules! deps {
    () => {
        Entry!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Debug for Entry { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Entry") . field ("result" , & self . result ()) . field ("user_data" , & self . user_data ()) . field ("flags" , & self . flags ()) . finish () } }
    };
}

impl_26!();