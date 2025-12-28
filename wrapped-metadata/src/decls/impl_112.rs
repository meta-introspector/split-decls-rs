macro_rules! deps {
    () => {
        ModuleRef!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl ModuleRef < '_ > { pub fn name (& self) -> & str { self . str (0) } }
    };
}

impl_112!();