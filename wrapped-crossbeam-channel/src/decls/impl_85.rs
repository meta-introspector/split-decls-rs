macro_rules! deps {
    () => {
        RecvError!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl error :: Error for RecvError { }
    };
}

impl_85!();