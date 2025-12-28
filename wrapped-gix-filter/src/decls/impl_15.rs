macro_rules! deps {
    () => {
        AutoCrlf!();
        Configuration!();
        Mode!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Configuration { # [doc = " Return the line-ending mode that is configured here."] pub fn to_eol (& self) -> Mode { match self . auto_crlf { AutoCrlf :: Enabled => Mode :: CrLf , AutoCrlf :: Input => Mode :: Lf , AutoCrlf :: Disabled => self . eol . unwrap_or_default () , } } }
    };
}

impl_15!()