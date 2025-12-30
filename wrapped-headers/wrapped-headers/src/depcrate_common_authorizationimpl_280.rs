// Generated macro for impl_280 (impl)
macro_rules! Depcrate_common_authorizationimpl_280 {
() => {
// Module: crate::common::authorization
// Provides: {"impl_280"}
// Dependencies: {}
impl Authorization < Bearer > { # [doc = " Try to create a `Bearer` authorization header."] pub fn bearer (token : & str) -> Result < Self , InvalidBearerToken > { HeaderValueString :: from_string (format ! ("Bearer {}" , token)) . map (| val | Authorization (Bearer (val))) . ok_or (InvalidBearerToken { _inner : () }) } # [doc = " View the token part as a `&str`."] pub fn token (& self) -> & str { self . 0 . token () } }
};
}
