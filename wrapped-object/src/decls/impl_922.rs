macro_rules! deps {
    () => {
        Export!();
    };
}

macro_rules! impl_922 {
    () => {
        deps!();
        impl < 'data > Export < 'data > { # [doc = " The symbol name."] # [inline] pub fn name (& self) -> & 'data [u8] { self . name . 0 } # [doc = " The virtual address of the symbol."] # [inline] pub fn address (& self) -> u64 { self . address } }
    };
}

impl_922!()