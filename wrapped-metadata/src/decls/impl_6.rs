macro_rules! impl_6 {
    () => {
        impl MethodAttributes { pub const Abstract : Self = Self (0x400) ; pub const HideBySig : Self = Self (0x80) ; pub const NewSlot : Self = Self (0x100) ; pub const Public : Self = Self (0x6) ; pub const SpecialName : Self = Self (0x800) ; pub const Virtual : Self = Self (0x40) ; }
    };
}

impl_6!()