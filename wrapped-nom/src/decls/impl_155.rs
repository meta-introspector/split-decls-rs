macro_rules! deps {
    () => {
        Err!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < E : Eq > Eq for Err < E > { }
    };
}

impl_155!();