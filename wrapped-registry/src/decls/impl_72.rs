macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < u64 > for Value { fn from (from : u64) -> Self { Self { data : from . to_le_bytes () . into () , ty : Type :: U64 , } } }
    };
}

impl_72!();