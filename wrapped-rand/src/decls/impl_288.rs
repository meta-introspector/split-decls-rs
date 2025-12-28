macro_rules! deps {
    () => {
        IndexedRandom!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl < T > IndexedRandom for [T] { fn len (& self) -> usize { self . len () } }
    };
}

impl_288!()