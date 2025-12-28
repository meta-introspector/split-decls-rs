macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        unsafe impl < T : ? Sized + Sync + Send > Send for Arc < T > { }
    };
}

impl_133!()