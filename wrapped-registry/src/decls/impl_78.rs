macro_rules! deps {
    () => {
        Data!();
        Value!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl From < & HSTRING > for Value { fn from (from : & HSTRING) -> Self { Self { data : Data :: from_slice (as_bytes (from)) , ty : Type :: String , } } }
    };
}

impl_78!()