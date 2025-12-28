macro_rules! deps {
    () => {
        RequestId!();
        IdRepr!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl From < i32 > for RequestId { fn from (id : i32) -> RequestId { RequestId (IdRepr :: I32 (id)) } }
    };
}

impl_16!();