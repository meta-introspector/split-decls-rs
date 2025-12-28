macro_rules! deps {
    () => {
        SignedData!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl SignedData < '_ > { # [doc = " Convenience method to obtain a copy of the signed data."] pub fn to_bstring (& self) -> BString { let mut buf = BString :: from (& self . data [.. self . signature_range . start]) ; buf . extend_from_slice (& self . data [self . signature_range . end ..]) ; buf } }
    };
}

impl_27!();