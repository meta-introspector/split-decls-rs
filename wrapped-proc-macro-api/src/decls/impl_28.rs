macro_rules! deps {
    () => {
        ServerError!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl fmt :: Display for ServerError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . message . fmt (f) ? ; if let Some (io) = & self . io { f . write_str (": ") ? ; io . fmt (f) ? ; } Ok (()) } }
    };
}

impl_28!()