macro_rules! deps {
    () => {
        ArcInner!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync + Send > Send for ArcInner < T > { }
    };
}

impl_130!()