macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl From < & str > for Value { fn from (v : & str) -> Self { Value (v . as_bytes () . into ()) } }
    };
}

impl_18!()