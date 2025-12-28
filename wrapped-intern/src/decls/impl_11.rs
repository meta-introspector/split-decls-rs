macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Eq for Interned < str > { }
    };
}

impl_11!()