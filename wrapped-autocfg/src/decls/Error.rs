macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " A common error type for the `autocfg` crate."] # [derive (Debug)] pub struct Error { kind : ErrorKind , }
    };
}

Error!();