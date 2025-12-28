macro_rules! deps {
    () => {
        Sender!();
        Value!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl Sender { pub (crate) fn send (& mut self , value : Value) { if self . shared . value . swap (value , Ordering :: SeqCst) != value { self . shared . waker . wake () ; } } }
    };
}

impl_95!()