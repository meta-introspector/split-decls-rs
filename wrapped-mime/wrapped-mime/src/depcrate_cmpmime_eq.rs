// Generated macro for mime_eq (function)
macro_rules! Depcrate_cmpmime_eq {
() => {
// Module: crate::cmp
// Provides: {"mime_eq"}
// Dependencies: {}
pub (crate) fn mime_eq (a : & Mime , b : & Mime) -> bool { match (a . private_atom () , b . private_atom ()) { (0 , _) | (_ , 0) => { essence_eq (a , b) && params_eq (a , b) } , (aa , ba) => aa == ba , } }
};
}
