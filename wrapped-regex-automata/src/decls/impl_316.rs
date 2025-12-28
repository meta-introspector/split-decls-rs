macro_rules! deps {
    () => {
        RetryFailError!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl RetryFailError { pub (crate) fn from_offset (offset : usize) -> RetryFailError { RetryFailError { offset } } }
    };
}

impl_316!()