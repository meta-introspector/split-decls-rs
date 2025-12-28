macro_rules! deps {
    () => {
        BufferKind!();
        BlockBuffer!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < BS : ArraySize , K : BufferKind > ZeroizeOnDrop for BlockBuffer < BS , K > { }
    };
}

impl_34!();