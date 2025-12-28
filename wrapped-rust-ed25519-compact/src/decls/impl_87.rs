macro_rules! deps {
    () => {
        Noise!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Deref for Noise { type Target = [u8 ; Noise :: BYTES] ; # [doc = " Returns the noise as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_87!();