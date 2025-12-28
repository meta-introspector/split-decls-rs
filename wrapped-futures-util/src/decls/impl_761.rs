macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_761 {
    () => {
        deps!();
        impl < T > Unpin for Empty < T > { }
    };
}

impl_761!();