macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! DurationInMilliseconds {
    () => {
        deps!();
        # [doc = " Keys specifying durations in milliseconds."] pub type DurationInMilliseconds = Any < validate :: DurationInMilliseconds > ;
    };
}

DurationInMilliseconds!();