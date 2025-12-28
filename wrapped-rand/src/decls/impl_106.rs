macro_rules! deps {
    () => {
        IntAsSIMD!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl IntAsSIMD for u64 { }
    };
}

impl_106!()