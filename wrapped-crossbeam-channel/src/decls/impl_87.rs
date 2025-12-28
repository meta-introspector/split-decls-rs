macro_rules! deps {
    () => {
        TryRecvError!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl error :: Error for TryRecvError { }
    };
}

impl_87!()