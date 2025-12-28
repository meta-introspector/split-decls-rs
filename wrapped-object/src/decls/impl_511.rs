macro_rules! deps {
    () => {
        ExportData!();
        ExportSymbol!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < 'data > ExportSymbol < 'data > { # [doc = " The name of the exported symbol."] pub fn name (& self) -> & [u8] { & self . name } # [doc = " The flags for the exported symbol."] pub fn flags (& self) -> u8 { self . flags } # [doc = " The terminal data for the exported symbol."] pub fn data (& self) -> & ExportData < 'data > { & self . data } }
    };
}

impl_511!();