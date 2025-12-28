macro_rules! deps {
    () => {
        RecvError!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl std :: error :: Error for RecvError { }
    };
}

impl_45!()