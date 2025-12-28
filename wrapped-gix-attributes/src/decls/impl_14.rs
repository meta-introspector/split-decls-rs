macro_rules! deps {
    () => {
        ValueRef!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl < 'a > ValueRef < 'a > { # [doc = " Keep `input` as our value."] pub fn from_bytes (input : & 'a [u8]) -> Self { Self (input) } }
    };
}

impl_14!()