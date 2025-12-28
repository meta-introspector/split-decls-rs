macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_538 {
    () => {
        deps!();
        impl < I > FusedIterator for Unique < I > where I : FusedIterator , I :: Item : Eq + Hash + Clone , { }
    };
}

impl_538!()