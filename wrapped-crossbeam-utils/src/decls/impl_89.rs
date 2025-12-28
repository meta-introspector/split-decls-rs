macro_rules! deps {
    () => {
        OnceLock!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        unsafe impl < T : Sync + Send > Sync for OnceLock < T > { }
    };
}

impl_89!()