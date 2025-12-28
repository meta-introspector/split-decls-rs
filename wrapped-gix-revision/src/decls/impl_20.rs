macro_rules! deps {
    () => {
        GenThenTime!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Eq for GenThenTime { }
    };
}

impl_20!()