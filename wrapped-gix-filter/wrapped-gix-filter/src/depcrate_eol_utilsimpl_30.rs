// Generated macro for impl_30 (impl)
macro_rules! Depcrate_eol_utilsimpl_30 {
() => {
// Module: crate::eol::utils
// Provides: {"impl_30"}
// Dependencies: {}
impl Configuration { # [doc = " Return the line-ending mode that is configured here."] pub fn to_eol (& self) -> Mode { match self . auto_crlf { AutoCrlf :: Enabled => Mode :: CrLf , AutoCrlf :: Input => Mode :: Lf , AutoCrlf :: Disabled => self . eol . unwrap_or_default () , } } }
};
}
