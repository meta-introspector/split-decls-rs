macro_rules! deps {
    () => {
        IterHashMut!();
    };
}

macro_rules! impl_513 {
    () => {
        deps!();
        impl < T > FusedIterator for IterHashMut < '_ , T > { }
    };
}

impl_513!();