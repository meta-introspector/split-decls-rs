macro_rules! deps {
    () => {
        RcVec!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < T > RefUnwindSafe for RcVec < T > where T : RefUnwindSafe { }
    };
}

impl_69!();