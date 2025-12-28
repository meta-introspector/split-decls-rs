macro_rules! deps {
    () => {
        Seed!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Deref for Seed { type Target = [u8 ; Seed :: BYTES] ; # [doc = " Returns a seed as raw bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_5!()