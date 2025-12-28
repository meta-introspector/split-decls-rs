macro_rules! deps {
    () => {
        PotentialUtf8!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Deref for PotentialUtf8 { type Target = [u8] ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_30!()