macro_rules! deps {
    () => {
        PollTimeout!();
        Error!();
    };
}

macro_rules! PollTimeoutTryFromError {
    () => {
        deps!();
        # [doc = " Error type for integer conversions into `PollTimeout`."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum PollTimeoutTryFromError { # [doc = " Passing a value less than -1 is invalid on some systems, see"] # [doc = " <https://man.freebsd.org/cgi/man.cgi?poll#end>."] TooNegative , # [doc = " Passing a value greater than `i32::MAX` is invalid."] TooPositive , }
    };
}

PollTimeoutTryFromError!();