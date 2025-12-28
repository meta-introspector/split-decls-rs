macro_rules! deps {
    () => {
        ArcInner!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync + Send > Sync for ArcInner < T > { }
    };
}

impl_131!();