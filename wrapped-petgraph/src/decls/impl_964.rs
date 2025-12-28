macro_rules! deps {
    () => {
        IterUtilsExt!();
    };
}

macro_rules! impl_964 {
    () => {
        deps!();
        impl < I > IterUtilsExt for I where I : Iterator { }
    };
}

impl_964!();