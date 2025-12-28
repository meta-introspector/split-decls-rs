macro_rules! deps {
    () => {
        Diff!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        unsafe impl < 'repo > Send for Diff < 'repo > { }
    };
}

impl_301!()