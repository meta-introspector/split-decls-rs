macro_rules! deps {
    () => {
        Reference!();
    };
}

macro_rules! impl_608 {
    () => {
        deps!();
        impl < 'repo > Eq for Reference < 'repo > { }
    };
}

impl_608!()