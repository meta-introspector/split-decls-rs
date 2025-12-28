macro_rules! deps {
    () => {
        GenericParam!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < 'a > GenericParam < 'a > { pub fn sequence (& self) -> u16 { self . usize (0) . try_into () . unwrap () } pub fn flags (& self) -> GenericParamAttributes { GenericParamAttributes (self . usize (1) . try_into () . unwrap ()) } pub fn owner (& self) -> TypeOrMethodDef < 'a > { self . decode (2) } pub fn name (& self) -> & str { self . str (3) } }
    };
}

impl_92!()