macro_rules! deps {
    () => {
        EscapeError!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl EscapeError { # [doc = " Returns true for actual errors, as opposed to warnings."] pub fn is_fatal (& self) -> bool { ! matches ! (self , EscapeError :: UnskippedWhitespaceWarning | EscapeError :: MultipleSkippedLinesWarning) } }
    };
}

impl_154!();