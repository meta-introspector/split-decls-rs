macro_rules! deps {
    () => {
        TryReserveErrorKind!();
        TryReserveError!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl From < TryReserveErrorKind > for TryReserveError { # [inline (always)] fn from (kind : TryReserveErrorKind) -> Self { Self { kind } } }
    };
}

impl_84!();