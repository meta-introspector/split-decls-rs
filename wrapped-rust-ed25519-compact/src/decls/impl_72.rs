macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Deref for Signature { type Target = [u8 ; Signature :: BYTES] ; # [doc = " Returns a signture as bytes."] fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_72!();