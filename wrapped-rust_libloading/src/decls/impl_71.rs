macro_rules! deps {
    () => {
        DlError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl core :: fmt :: Display for DlError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . write_str (& self . 0 . to_string_lossy ()) } }
    };
}

impl_71!();