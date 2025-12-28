macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl TryFrom < Value > for u64 { type Error = Error ; fn try_from (from : Value) -> Result < Self > { from_le_bytes (from . ty , & from) } }
    };
}

impl_73!();