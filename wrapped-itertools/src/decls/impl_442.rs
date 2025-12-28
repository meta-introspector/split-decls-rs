macro_rules! deps {
    () => {
        Powerset!();
    };
}

macro_rules! impl_442 {
    () => {
        deps!();
        impl < I > FusedIterator for Powerset < I > where I : Iterator , I :: Item : Clone , { }
    };
}

impl_442!();