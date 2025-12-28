macro_rules! deps {
    () => {
        Interface!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl Eq for Interface { }
    };
}

impl_303!();