// Generated macro for impl_595 (impl)
macro_rules! Depcrate_common_set_cookieimpl_595 {
() => {
// Module: crate::common::set_cookie
// Provides: {"impl_595"}
// Dependencies: {}
impl Header for SetCookie { fn name () -> & 'static HeaderName { & :: http :: header :: SET_COOKIE } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { let vec = values . cloned () . collect :: < Vec < _ > > () ; if ! vec . is_empty () { Ok (SetCookie (vec)) } else { Err (Error :: invalid ()) } } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (self . 0 . iter () . cloned ()) ; } }
};
}
