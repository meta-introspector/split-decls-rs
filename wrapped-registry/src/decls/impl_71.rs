macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl TryFrom < Value > for u32 { type Error = Error ; fn try_from (from : Value) -> Result < Self > { Ok (from_le_bytes (from . ty , & from) ? . try_into () ?) } }
    };
}

impl_71!()