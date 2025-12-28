macro_rules! deps {
    () => {
        Span!();
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl Error { # [doc = " Return the type of this error."] pub fn kind (& self) -> & ErrorKind { & self . kind } # [doc = " The original pattern string in which this error occurred."] # [doc = ""] # [doc = " Every span reported by this error is reported in terms of this string."] pub fn pattern (& self) -> & str { & self . pattern } # [doc = " Return the span at which this error occurred."] pub fn span (& self) -> & Span { & self . span } }
    };
}

impl_213!();