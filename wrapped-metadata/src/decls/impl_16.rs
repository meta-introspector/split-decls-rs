macro_rules! impl_16 {
    () => {
        impl TypeAttributes { pub const Public : Self = Self (0x1) ; pub const ExplicitLayout : Self = Self (0x10) ; pub const Abstract : Self = Self (0x80) ; pub const Sealed : Self = Self (0x100) ; pub const WindowsRuntime : Self = Self (0x4000) ; pub const Interface : Self = Self (0x20) ; pub const SequentialLayout : Self = Self (0x8) ; pub const Import : Self = Self (0x1000) ; pub fn is_nested (& self) -> bool { (self . 0 & 0x00000006) != 0 } }
    };
}

impl_16!()