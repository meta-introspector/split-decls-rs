macro_rules! deps {
    () => {
        ComPtr!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Eq for ComPtr { }
    };
}

impl_45!();