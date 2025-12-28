macro_rules! deps {
    () => {
        Error!();
        ShareError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl error :: Error for ShareError { }
    };
}

impl_9!()