macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < F > Unpin for PollFn < F > { }
    };
}

impl_180!();