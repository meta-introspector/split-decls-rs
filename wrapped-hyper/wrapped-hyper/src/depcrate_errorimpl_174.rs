// Generated macro for impl_174 (impl)
macro_rules! Depcrate_errorimpl_174 {
() => {
// Module: crate::error
// Provides: {"impl_174"}
// Dependencies: {}
# [cfg (feature = "http1")] impl From < httparse :: Error > for Parse { fn from (err : httparse :: Error) -> Parse { match err { httparse :: Error :: HeaderName | httparse :: Error :: HeaderValue | httparse :: Error :: NewLine | httparse :: Error :: Token => Parse :: Header (Header :: Token) , httparse :: Error :: Status => Parse :: Status , httparse :: Error :: TooManyHeaders => Parse :: TooLarge , httparse :: Error :: Version => Parse :: Version , } } }
};
}
