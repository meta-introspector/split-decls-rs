macro_rules! deps {
    () => {
        WrappedWaker!();
        Send!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        unsafe impl Send for WrappedWaker { }
    };
}

impl_528!()