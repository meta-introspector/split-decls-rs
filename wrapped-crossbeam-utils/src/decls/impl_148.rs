macro_rules! deps {
    () => {
        WaitGroup!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl Clone for WaitGroup { fn clone (& self) -> Self { let mut count = self . inner . count . lock () . unwrap () ; * count += 1 ; Self { inner : self . inner . clone () , } } }
    };
}

impl_148!()