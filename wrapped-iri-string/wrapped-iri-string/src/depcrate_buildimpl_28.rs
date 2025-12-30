// Generated macro for impl_28 (impl)
macro_rules! Depcrate_buildimpl_28 {
() => {
// Module: crate::build
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > From < (& 'a str , & 'a str) > for UserinfoBuilder < 'a > { # [inline] fn from ((user , password) : (& 'a str , & 'a str)) -> Self { Self (UserinfoRepr :: UserPass (user , Some (password))) } }
};
}
