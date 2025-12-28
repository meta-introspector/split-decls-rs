macro_rules! deps {
    () => {
        UnparkMutex!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        unsafe impl < D : Send > Send for UnparkMutex < D > { }
    };
}

impl_44!()