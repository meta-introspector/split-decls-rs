macro_rules! impl_96 {
    () => {
        impl MethodParam { pub fn flags (& self) -> ParamAttributes { ParamAttributes (self . usize (0) as u16) } pub fn sequence (& self) -> u16 { self . usize (1) as u16 } pub fn name (& self) -> & 'static str { self . str (2) } }
    };
}

impl_96!();