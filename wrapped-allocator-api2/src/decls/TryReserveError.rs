macro_rules! deps {
    () => {
        TryReserveErrorKind!();
    };
}

macro_rules! TryReserveError {
    () => {
        deps!();
        # [doc = " The error type for `try_reserve` methods."] # [derive (Clone , PartialEq , Eq , Debug)] pub struct TryReserveError { kind : TryReserveErrorKind , }
    };
}

TryReserveError!();