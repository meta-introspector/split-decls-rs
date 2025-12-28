macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl From < String > for Bytes { fn from (s : String) -> Bytes { Bytes :: from (s . into_bytes ()) } }
    };
}

impl_112!();