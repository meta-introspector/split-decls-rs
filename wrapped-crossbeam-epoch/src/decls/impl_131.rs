macro_rules! deps {
    () => {
        OnceLock!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        unsafe impl < T : Sync + Send > Sync for OnceLock < T > { }
    };
}

impl_131!();