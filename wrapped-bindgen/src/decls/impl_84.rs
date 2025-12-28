macro_rules! impl_84 {
    () => {
        impl ImplMap { pub fn flags (& self) -> PInvokeAttributes { PInvokeAttributes (self . usize (0)) } pub fn scope (& self) -> ModuleRef { ModuleRef (self . row (3)) } pub fn import_name (& self) -> & 'static str { self . str (2) } }
    };
}

impl_84!();