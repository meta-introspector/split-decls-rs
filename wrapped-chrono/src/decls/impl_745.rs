macro_rules! deps {
    () => {
        OutOfRange!();
    };
}

macro_rules! impl_745 {
    () => {
        deps!();
        impl OutOfRange { const fn new () -> OutOfRange { OutOfRange { _private : () } } }
    };
}

impl_745!();