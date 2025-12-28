macro_rules! deps {
    () => {
        ParseError!();
        ParseErrorKind!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl ParseError { # [doc = " An invalid hex flag was encountered."] pub fn invalid_hex_flag (flag : impl fmt :: Display) -> Self { let _flag = flag ; let got = { # [cfg (feature = "std")] { _flag . to_string () } } ; ParseError (ParseErrorKind :: InvalidHexFlag { got }) } # [doc = " A named flag that doesn't correspond to any on the flags type was encountered."] pub fn invalid_named_flag (flag : impl fmt :: Display) -> Self { let _flag = flag ; let got = { # [cfg (feature = "std")] { _flag . to_string () } } ; ParseError (ParseErrorKind :: InvalidNamedFlag { got }) } # [doc = " A hex or named flag wasn't found between separators."] pub const fn empty_flag () -> Self { ParseError (ParseErrorKind :: EmptyFlag) } }
    };
}

impl_124!()