macro_rules! deps {
    () => {
        BufferKind!();
        BlockBuffer!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < BS : ArraySize , K : BufferKind > Clone for BlockBuffer < BS , K > { # [inline] fn clone (& self) -> Self { unsafe { ptr :: read (self) } } }
    };
}

impl_14!()