// Generated macro for impl_29 (impl)
macro_rules! Depcrate_buildimpl_29 {
() => {
// Module: crate::build
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a > From < (& 'a str , Option < & 'a str >) > for UserinfoBuilder < 'a > { # [inline] fn from ((user , password) : (& 'a str , Option < & 'a str >)) -> Self { Self (UserinfoRepr :: UserPass (user , password)) } }
};
}
