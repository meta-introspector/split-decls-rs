macro_rules! impl_348 {
    () => {
        impl TypeAttributes { pub const ExplicitLayout : Self = Self (0x10) ; pub const WindowsRuntime : Self = Self (0x4000) ; pub fn is_nested (& self) -> bool { (self . 0 & 0x00000006) != 0 } }
    };
}

impl_348!();