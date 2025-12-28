macro_rules! Sealed {
    () => {
        pub trait Sealed { }
    };
}

Sealed!();