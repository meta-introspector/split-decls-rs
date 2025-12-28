macro_rules! deps {
    () => {
        Window!();
    };
}

macro_rules! impl_1224 {
    () => {
        deps!();
        impl < T : AsMut < [u8] > > AsMut < [u8] > for Window < T > { fn as_mut (& mut self) -> & mut [u8] { & mut self . inner . as_mut () [self . range . start .. self . range . end] } }
    };
}

impl_1224!();