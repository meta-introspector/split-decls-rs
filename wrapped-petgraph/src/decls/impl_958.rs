macro_rules! deps {
    () => {
        IterFormatExt!();
    };
}

macro_rules! impl_958 {
    () => {
        deps!();
        impl < I > IterFormatExt for I where I : Iterator { }
    };
}

impl_958!();