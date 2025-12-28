macro_rules! StringTypedError {
    () => {
        # [doc = " String-based error type with an optional source"] # [derive (Debug)] struct StringTypedError { message : String , source : Option < Box < StringTypedError > > , }
    };
}

StringTypedError!();