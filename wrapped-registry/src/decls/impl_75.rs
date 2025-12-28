macro_rules! deps {
    () => {
        Value!();
        Data!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl From < & str > for Value { fn from (from : & str) -> Self { Self { data : Data :: from_slice (pcwstr (from) . as_bytes ()) , ty : Type :: String , } } }
    };
}

impl_75!()