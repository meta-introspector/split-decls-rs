macro_rules! deps {
    () => {
        BlockingStream!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < S : Stream + Unpin > BlockingStream < S > { # [doc = " Convert this `BlockingStream` into the inner `Stream` type."] pub fn into_inner (self) -> S { self . stream } }
    };
}

impl_16!()