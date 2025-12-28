macro_rules! deps {
    () => {
        ReadBuffer!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < BS : ArraySize > zeroize :: ZeroizeOnDrop for ReadBuffer < BS > { }
    };
}

impl_6!();