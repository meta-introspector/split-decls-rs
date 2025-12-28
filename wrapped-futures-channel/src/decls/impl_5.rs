macro_rules! deps {
    () => {
        Lock!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Lock < T > { }
    };
}

impl_5!()