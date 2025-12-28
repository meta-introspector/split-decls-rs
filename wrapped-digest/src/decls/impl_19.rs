macro_rules! deps {
    () => {
        InvalidBufferSize!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl fmt :: Display for InvalidBufferSize { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str ("invalid buffer length") } }
    };
}

impl_19!()