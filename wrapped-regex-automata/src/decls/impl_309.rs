macro_rules! deps {
    () => {
        RetryError!();
        RetryFailError!();
        MatchError!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        impl From < MatchError > for RetryError { fn from (merr : MatchError) -> RetryError { RetryError :: Fail (RetryFailError :: from (merr)) } }
    };
}

impl_309!();