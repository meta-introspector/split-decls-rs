macro_rules! deps {
    () => {
        Protocol!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [cfg (feature = "http2")] impl AsRef < [u8] > for Protocol { fn as_ref (& self) -> & [u8] { self . inner . as_ref () } }
    };
}

impl_152!();