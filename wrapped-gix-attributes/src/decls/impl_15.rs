macro_rules! deps {
    () => {
        Value!();
        ValueRef!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [doc = " Access and conversions"] impl < 'a > ValueRef < 'a > { # [doc = " Access this value as byte string."] pub fn as_bstr (& self) -> & 'a BStr { self . 0 . as_bytes () . as_bstr () } # [doc = " Convert this instance into its owned form."] pub fn to_owned (self) -> Value { self . into () } }
    };
}

impl_15!()