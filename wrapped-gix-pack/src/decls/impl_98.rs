macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl From < TryReserveError > for Error { # [cold] fn from (_ : TryReserveError) -> Self { Self :: OutOfMemory } }
    };
}

impl_98!();