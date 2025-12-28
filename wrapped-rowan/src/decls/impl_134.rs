macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync + Send > Sync for Arc < T > { }
    };
}

impl_134!()