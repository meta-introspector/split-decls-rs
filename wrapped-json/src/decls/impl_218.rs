macro_rules! deps {
    () => {
        PrettyFormatter!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'a > PrettyFormatter < 'a > { # [doc = " Construct a pretty printer formatter that defaults to using two spaces for indentation."] pub fn new () -> Self { PrettyFormatter :: with_indent (b"  ") } # [doc = " Construct a pretty printer formatter that uses the `indent` string for indentation."] pub fn with_indent (indent : & 'a [u8]) -> Self { PrettyFormatter { current_indent : 0 , has_value : false , indent , } } }
    };
}

impl_218!();