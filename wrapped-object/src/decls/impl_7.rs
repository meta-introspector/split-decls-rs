macro_rules! deps {
    () => {
        AddressSize!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl AddressSize { # [doc = " The size in bytes of an address value."] # [inline] pub fn bytes (self) -> u8 { self as u8 } }
    };
}

impl_7!();