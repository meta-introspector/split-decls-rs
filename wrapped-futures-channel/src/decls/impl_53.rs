macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl std :: error :: Error for TryRecvError { }
    };
}

impl_53!()