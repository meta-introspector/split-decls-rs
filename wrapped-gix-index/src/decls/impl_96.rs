macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl From < FileTime > for Time { fn from (value : FileTime) -> Self { Time { secs : value . unix_seconds () . try_into () . expect ("can't represent non-unix times") , nsecs : value . nanoseconds () , } } }
    };
}

impl_96!();