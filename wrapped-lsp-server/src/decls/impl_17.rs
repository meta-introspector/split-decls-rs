macro_rules! deps {
    () => {
        RequestId!();
        IdRepr!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < String > for RequestId { fn from (id : String) -> RequestId { RequestId (IdRepr :: String (id)) } }
    };
}

impl_17!()