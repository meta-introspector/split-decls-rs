macro_rules! deps {
    () => {
        SendError!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl std :: error :: Error for SendError { }
    };
}

impl_43!();