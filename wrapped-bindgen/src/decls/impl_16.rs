macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Config < '_ > { pub fn with_namespace (& self , namespace : & 'static str) -> Self { let mut clone = self . clone () ; clone . namespace = namespace ; clone } }
    };
}

impl_16!()