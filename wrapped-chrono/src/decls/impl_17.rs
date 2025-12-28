macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl OutOfRange { const fn new () -> OutOfRange { OutOfRange { _private : () } } }
    };
}

impl_17!()