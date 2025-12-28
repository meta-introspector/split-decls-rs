macro_rules! deps {
    () => {
        Import!();
    };
}

macro_rules! impl_920 {
    () => {
        deps!();
        impl < 'data > Import < 'data > { # [doc = " The symbol name."] # [inline] pub fn name (& self) -> & 'data [u8] { self . name . 0 } # [doc = " The name of the library to import the symbol from."] # [inline] pub fn library (& self) -> & 'data [u8] { self . library . 0 } }
    };
}

impl_920!();