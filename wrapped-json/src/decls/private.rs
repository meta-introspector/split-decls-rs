macro_rules! private {
    () => {
        mod private { pub trait Sealed { } }
    };
}

private!();