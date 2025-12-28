macro_rules! deps {
    () => {
        Producer!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        unsafe impl < T > Send for Producer < '_ , T > where T : Send { }
    };
}

impl_477!()