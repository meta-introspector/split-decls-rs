macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl Eq for Encoding { }
    };
}

impl_499!();