macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < I > Error < I > { # [doc = " creates a new basic error"] pub fn new (input : I , code : ErrorKind) -> Error < I > { Error { input , code } } }
    };
}

impl_10!()