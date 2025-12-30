// Generated macro for impl_102 (impl)
macro_rules! Depcrateimpl_102 {
() => {
// Module: crate
// Provides: {"impl_102"}
// Dependencies: {}
impl Error { # [inline] fn description_str (& self) -> & 'static str { match * self { Error :: HeaderName => "invalid header name" , Error :: HeaderValue => "invalid header value" , Error :: NewLine => "invalid new line" , Error :: Status => "invalid response status" , Error :: Token => "invalid token" , Error :: TooManyHeaders => "too many headers" , Error :: Version => "invalid HTTP version" , } } }
};
}
