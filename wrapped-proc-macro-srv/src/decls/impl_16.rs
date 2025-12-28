macro_rules! deps {
    () => {
        PanicMessage!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl PanicMessage { pub fn into_string (self) -> Option < String > { self . message } }
    };
}

impl_16!()