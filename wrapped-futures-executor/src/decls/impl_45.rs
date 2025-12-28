macro_rules! deps {
    () => {
        UnparkMutex!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        unsafe impl < D : Send > Sync for UnparkMutex < D > { }
    };
}

impl_45!()