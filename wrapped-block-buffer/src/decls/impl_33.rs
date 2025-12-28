macro_rules! deps {
    () => {
        BlockBuffer!();
        BufferKind!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < BS : ArraySize , K : BufferKind > Drop for BlockBuffer < BS , K > { # [inline] fn drop (& mut self) { # [cfg (feature = "zeroize")] self . zeroize () ; } }
    };
}

impl_33!();