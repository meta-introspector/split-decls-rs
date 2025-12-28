macro_rules! impl_2 {
    () => {
        impl AssemblyFlags { pub const WindowsRuntime : Self = Self (0x200) ; }
    };
}

impl_2!();