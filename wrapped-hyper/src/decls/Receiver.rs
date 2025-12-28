macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! Receiver {
    () => {
        deps!();
        pub (crate) struct Receiver { shared : Arc < Shared > , }
    };
}

Receiver!()