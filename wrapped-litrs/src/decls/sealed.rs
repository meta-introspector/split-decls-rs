macro_rules! sealed {
    () => {
        mod sealed { pub trait Sealed { } }
    };
}

sealed!();