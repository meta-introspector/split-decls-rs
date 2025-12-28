macro_rules! deps {
    () => {
        PushError!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Display for PushError { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { f . write_str ("submission queue is full") } }
    };
}

impl_149!();