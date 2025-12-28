macro_rules! deps {
    () => {
        HeapVec!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl Eq for HeapVec { }
    };
}

impl_63!();