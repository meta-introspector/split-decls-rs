macro_rules! deps {
    () => {
        PollTimeoutTryFromError!();
        Error!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl std :: error :: Error for PollTimeoutTryFromError { }
    };
}

impl_247!()