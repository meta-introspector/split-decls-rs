macro_rules! deps {
    () => {
        CertX509!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < 'a > CertX509 < 'a > { # [doc = " Return the X.509 certificate data as a byte slice"] pub fn data (& self) -> & [u8] { unsafe { slice :: from_raw_parts ((* self . raw) . data as * const u8 , (* self . raw) . len as usize) } } }
    };
}

impl_114!()