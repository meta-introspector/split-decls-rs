macro_rules! deps {
    () => {
        IteratorRandom!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < I > IteratorRandom for I where I : Iterator + Sized { }
    };
}

impl_282!()