macro_rules! deps {
    () => {
        BlockBuffer!();
        BufferKind!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < BS : ArraySize , K : BufferKind > ZeroizeOnDrop for BlockBuffer < BS , K > { }
    };
}

impl_23!()