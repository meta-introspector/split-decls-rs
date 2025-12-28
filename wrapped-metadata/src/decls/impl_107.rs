macro_rules! impl_107 {
    () => {
        impl MethodParam < '_ > { pub fn flags (& self) -> ParamAttributes { ParamAttributes (self . usize (0) . try_into () . unwrap ()) } pub fn sequence (& self) -> u16 { self . usize (1) . try_into () . unwrap () } pub fn name (& self) -> & str { self . str (2) } }
    };
}

impl_107!();