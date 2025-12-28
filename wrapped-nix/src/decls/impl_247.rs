macro_rules! deps {
    () => {
        Error!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl std :: error :: Error for PollTimeoutTryFromError { }
    };
}

impl_247!();