macro_rules! deps {
    () => {
        IndexedMutRandom!();
        IndexedRandom!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < IR : IndexedRandom + IndexMut < usize > + ? Sized > IndexedMutRandom for IR { }
    };
}

impl_289!()