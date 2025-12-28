macro_rules! deps {
    () => {
        Send!();
        WrappedWaker!();
    };
}

macro_rules! impl_528 {
    () => {
        deps!();
        unsafe impl Send for WrappedWaker { }
    };
}

impl_528!();