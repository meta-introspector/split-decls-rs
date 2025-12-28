macro_rules! deps {
    () => {
        TextRef!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a > TextRef < 'a > { # [doc = " Return this instance's data."] pub fn as_slice (& self) -> & 'a [u8] { self . 0 } # [doc = " Return this instance's data as [`BStr`]."] pub fn as_bstr (& self) -> & 'a BStr { self . 0 . into () } }
    };
}

impl_20!()