macro_rules! deps {
    () => {
        Task!();
        Send!();
    };
}

macro_rules! impl_857 {
    () => {
        deps!();
        unsafe impl < Fut > Send for Task < Fut > { }
    };
}

impl_857!()