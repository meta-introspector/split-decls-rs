macro_rules! deps {
    () => {
        Send!();
        Task!();
    };
}

macro_rules! impl_857 {
    () => {
        deps!();
        unsafe impl < Fut > Send for Task < Fut > { }
    };
}

impl_857!();