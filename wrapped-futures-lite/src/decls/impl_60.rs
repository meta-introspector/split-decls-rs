macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < F > Unpin for PollFn < F > { }
    };
}

impl_60!();