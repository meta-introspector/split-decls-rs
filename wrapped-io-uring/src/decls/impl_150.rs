macro_rules! deps {
    () => {
        PushError!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl Error for PushError { }
    };
}

impl_150!()