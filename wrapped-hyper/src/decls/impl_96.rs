macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Drop for Sender { fn drop (& mut self) { self . send (CLOSED) ; } }
    };
}

impl_96!();