macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! impl_525 {
    () => {
        deps!();
        impl Eq for Oid { }
    };
}

impl_525!()