macro_rules! deps {
    () => {
        StateRef!();
        ValueRef!();
        Value!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [doc = " Initialization"] impl < 'a > StateRef < 'a > { # [doc = " Keep `input` in one of our enums."] pub fn from_bytes (input : & 'a [u8]) -> Self { Self :: Value (ValueRef :: from_bytes (input)) } }
    };
}

impl_21!()