macro_rules! deps {
    () => {
        Commit!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl Eq for Commit < '_ > { }
    };
}

impl_17!()