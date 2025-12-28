macro_rules! deps {
    () => {
        TypeRef!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a > TypeRef < 'a > { pub fn scope (& self) -> ResolutionScope < 'a > { self . decode (0) } pub fn name (& self) -> & 'a str { self . str (1) } pub fn namespace (& self) -> & 'a str { self . str (2) } }
    };
}

impl_121!();