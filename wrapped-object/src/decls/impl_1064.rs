macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_1064 {
    () => {
        deps!();
        impl Error { pub (super) fn new (message : impl Into < String >) -> Self { Error (message . into ()) } }
    };
}

impl_1064!();