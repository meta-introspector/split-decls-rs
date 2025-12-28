macro_rules! Utf8Error {
    () => {
        # [derive (Debug)] # [doc = " The error type returned by [`into_bstr()`] and others may suffer from failed conversions from or to bytes."] pub struct Utf8Error ;
    };
}

Utf8Error!();