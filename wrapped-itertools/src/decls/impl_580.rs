macro_rules! deps {
    () => {
        ZipLongest!();
    };
}

macro_rules! impl_580 {
    () => {
        deps!();
        impl < T , U > FusedIterator for ZipLongest < T , U > where T : Iterator , U : Iterator , { }
    };
}

impl_580!();