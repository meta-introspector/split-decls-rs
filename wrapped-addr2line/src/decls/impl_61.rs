macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl < 'a > Symbol < 'a > { # [doc = " Get the symbol name."] pub fn name (& self) -> & 'a str { self . name } # [doc = " Get the symbol address."] pub fn address (& self) -> u64 { self . address } }
    };
}

impl_61!();