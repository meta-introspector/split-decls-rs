macro_rules! deps {
    () => {
        IdentIsRaw!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl IdentIsRaw { pub fn to_print_mode_ident (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawIdent , } } pub fn to_print_mode_lifetime (self) -> IdentPrintMode { match self { IdentIsRaw :: No => IdentPrintMode :: Normal , IdentIsRaw :: Yes => IdentPrintMode :: RawLifetime , } } }
    };
}

impl_406!()