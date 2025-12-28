macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl From < u32 > for Value { fn from (from : u32) -> Self { Self { data : from . to_le_bytes () . into () , ty : Type :: U32 , } } }
    };
}

impl_70!()