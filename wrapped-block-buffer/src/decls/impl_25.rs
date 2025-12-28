macro_rules! deps {
    () => {
        BlockBuffer!();
        BufferKind!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < BS : ArraySize , K : BufferKind > Clone for BlockBuffer < BS , K > { # [inline] fn clone (& self) -> Self { unsafe { ptr :: read (self) } } }
    };
}

impl_25!()