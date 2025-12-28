macro_rules! deps {
    () => {
        PollFn!();
    };
}

macro_rules! impl_780 {
    () => {
        deps!();
        impl < F > Unpin for PollFn < F > { }
    };
}

impl_780!();