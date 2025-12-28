macro_rules! deps {
    () => {
        StreamGroup!();
        Keyed!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        impl < S : Stream > Deref for Keyed < S > { type Target = StreamGroup < S > ; fn deref (& self) -> & Self :: Target { & self . group } }
    };
}

impl_397!()