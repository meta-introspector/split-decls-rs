macro_rules! deps {
    () => {
        ImplMap!();
        ModuleRef!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'a > ImplMap < 'a > { pub fn flags (& self) -> PInvokeAttributes { PInvokeAttributes (self . usize (0) . try_into () . unwrap ()) } pub fn import_name (& self) -> & str { self . str (2) } pub fn import_scope (& self) -> ModuleRef < 'a > { self . row (3) } }
    };
}

impl_95!();