macro_rules! deps {
    () => {
        RetryFailError!();
        RetryError!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl From < RetryFailError > for RetryError { fn from (err : RetryFailError) -> RetryError { RetryError :: Fail (err) } }
    };
}

impl_319!()