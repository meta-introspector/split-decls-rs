macro_rules! deps {
    () => {
        Lock!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Lock < T > { }
    };
}

impl_4!();