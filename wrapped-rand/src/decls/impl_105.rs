macro_rules! deps {
    () => {
        IntAsSIMD!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl IntAsSIMD for u32 { }
    };
}

impl_105!()