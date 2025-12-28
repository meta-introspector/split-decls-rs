macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < T > Unpin for Receiver < T > { }
    };
}

impl_104!();