macro_rules! deps {
    () => {
        RoundingError!();
    };
}

macro_rules! impl_691 {
    () => {
        deps!();
        impl fmt :: Display for RoundingError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { RoundingError :: DurationExceedsTimestamp => { write ! (f , "duration in nanoseconds exceeds timestamp") } RoundingError :: DurationExceedsLimit => { write ! (f , "duration exceeds num_nanoseconds limit") } RoundingError :: TimestampExceedsLimit => { write ! (f , "timestamp exceeds num_nanoseconds limit") } } } }
    };
}

impl_691!();