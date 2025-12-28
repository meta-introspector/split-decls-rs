macro_rules! deps {
    () => {
        AllowStdIo!();
    };
}

macro_rules! impl_1047 {
    () => {
        deps!();
        impl < T > Unpin for AllowStdIo < T > { }
    };
}

impl_1047!();