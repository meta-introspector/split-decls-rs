macro_rules! deps {
    () => {
        TryStreamExt!();
    };
}

macro_rules! impl_738 {
    () => {
        deps!();
        impl < S : ? Sized + TryStream > TryStreamExt for S { }
    };
}

impl_738!();