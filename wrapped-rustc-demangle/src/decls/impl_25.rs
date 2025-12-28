macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ParseError { # [doc = " Snippet to print when the error is initially encountered."] fn message (& self) -> & str { match self { ParseError :: Invalid => "{invalid syntax}" , ParseError :: RecursedTooDeep => "{recursion limit reached}" , } } }
    };
}

impl_25!()