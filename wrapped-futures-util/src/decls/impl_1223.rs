macro_rules! deps {
    () => {
        Window!();
    };
}

macro_rules! impl_1223 {
    () => {
        deps!();
        impl < T : AsRef < [u8] > > AsRef < [u8] > for Window < T > { fn as_ref (& self) -> & [u8] { & self . inner . as_ref () [self . range . start .. self . range . end] } }
    };
}

impl_1223!();