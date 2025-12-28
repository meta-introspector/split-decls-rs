macro_rules! deps {
    () => {
        ThreadPool!();
        AssertSendSync!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl AssertSendSync for ThreadPool { }
    };
}

impl_24!();