macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Eq for Interned < str > { }
    };
}

impl_36!();