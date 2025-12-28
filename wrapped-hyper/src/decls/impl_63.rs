macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < F > Unpin for PollFn < F > { }
    };
}

impl_63!();