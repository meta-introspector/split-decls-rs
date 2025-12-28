macro_rules! deps {
    () => {
        ZipLongest!();
    };
}

macro_rules! impl_579 {
    () => {
        deps!();
        impl < T , U > ExactSizeIterator for ZipLongest < T , U > where T : ExactSizeIterator , U : ExactSizeIterator , { }
    };
}

impl_579!();