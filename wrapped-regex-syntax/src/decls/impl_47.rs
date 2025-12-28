macro_rules! deps {
    () => {
        Span!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Error { # [doc = " Return the type of this error."] pub fn kind (& self) -> & ErrorKind { & self . kind } # [doc = " The original pattern string in which this error occurred."] # [doc = ""] # [doc = " Every span reported by this error is reported in terms of this string."] pub fn pattern (& self) -> & str { & self . pattern } # [doc = " Return the span at which this error occurred."] pub fn span (& self) -> & Span { & self . span } # [doc = " Return an auxiliary span. This span exists only for some errors that"] # [doc = " benefit from being able to point to two locations in the original"] # [doc = " regular expression. For example, \"duplicate\" errors will have the"] # [doc = " main error position set to the duplicate occurrence while its"] # [doc = " auxiliary span will be set to the initial occurrence."] pub fn auxiliary_span (& self) -> Option < & Span > { use self :: ErrorKind :: * ; match self . kind { FlagDuplicate { ref original } => Some (original) , FlagRepeatedNegation { ref original , .. } => Some (original) , GroupNameDuplicate { ref original , .. } => Some (original) , _ => None , } } }
    };
}

impl_47!();