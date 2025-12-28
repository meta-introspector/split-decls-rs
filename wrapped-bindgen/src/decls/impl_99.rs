macro_rules! impl_99 {
    () => {
        impl ModuleRef { pub fn name (& self) -> & 'static str { self . str (0) } }
    };
}

impl_99!();