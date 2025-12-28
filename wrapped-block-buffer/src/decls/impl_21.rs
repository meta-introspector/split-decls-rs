macro_rules! deps {
    () => {
        BlockBuffer!();
        BufferKind!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] impl < BS : ArraySize , K : BufferKind > Zeroize for BlockBuffer < BS , K > { # [inline] fn zeroize (& mut self) { self . buffer . zeroize () ; self . pos . zeroize () ; } }
    };
}

impl_21!()