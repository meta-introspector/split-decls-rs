macro_rules! deps {
    () => {
        Collector!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl Eq for Collector { }
    };
}

impl_74!();