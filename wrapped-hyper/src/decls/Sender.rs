macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! Sender {
    () => {
        deps!();
        pub (crate) struct Sender { shared : Arc < Shared > , }
    };
}

Sender!();