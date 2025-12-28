macro_rules! deps {
    () => {
        NotBothDebug!();
        Error!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < A , B > NotBothDebug for & (A , B) { fn __dispatch_ensure (self , msg : & 'static str) -> Error { Error :: msg (msg) } }
    };
}

impl_37!();