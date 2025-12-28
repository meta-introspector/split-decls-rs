macro_rules! deps {
    () => {
        IdRepr!();
        RequestId!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < String > for RequestId { fn from (id : String) -> RequestId { RequestId (IdRepr :: String (id)) } }
    };
}

impl_17!();