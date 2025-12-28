macro_rules! deps {
    () => {
        TryReserveError!();
        TryReserveErrorKind!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl TryReserveError { # [doc = " Details about the allocation that caused the error"] pub fn kind (& self) -> TryReserveErrorKind { self . kind . clone () } }
    };
}

impl_82!();