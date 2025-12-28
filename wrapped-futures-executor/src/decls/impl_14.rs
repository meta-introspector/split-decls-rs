macro_rules! deps {
    () => {
        BlockingStream!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < S : Stream + Unpin > Deref for BlockingStream < S > { type Target = S ; fn deref (& self) -> & Self :: Target { & self . stream } }
    };
}

impl_14!();