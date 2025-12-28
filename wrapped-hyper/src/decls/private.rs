macro_rules! private {
    () => {
        mod private { # ! [allow (missing_debug_implementations)] pub struct Sealed { } }
    };
}

private!();