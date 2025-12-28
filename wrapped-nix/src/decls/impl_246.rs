macro_rules! deps {
    () => {
        Result!();
        PollTimeoutTryFromError!();
    };
}

macro_rules! impl_246 {
    () => {
        deps!();
        impl std :: fmt :: Display for PollTimeoutTryFromError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: TooNegative => write ! (f , "Passed a negative timeout less than -1.") , Self :: TooPositive => write ! (f , "Passed a positive timeout greater than `i32::MAX` milliseconds.") } } }
    };
}

impl_246!();