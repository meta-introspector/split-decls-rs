macro_rules! deps {
    () => {
        ShareError!();
        Error!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl error :: Error for ShareError { }
    };
}

impl_9!();