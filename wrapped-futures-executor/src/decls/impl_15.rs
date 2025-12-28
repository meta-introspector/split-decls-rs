macro_rules! deps {
    () => {
        BlockingStream!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S : Stream + Unpin > DerefMut for BlockingStream < S > { fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . stream } }
    };
}

impl_15!()