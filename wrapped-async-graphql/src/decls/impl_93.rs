macro_rules! deps {
    () => {
        Registry!();
        Schema!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'a > __Schema < 'a > { pub fn new (registry : & 'a registry :: Registry , visible_types : & 'a HashSet < & 'a str >) -> Self { Self { registry , visible_types , } } }
    };
}

impl_93!();