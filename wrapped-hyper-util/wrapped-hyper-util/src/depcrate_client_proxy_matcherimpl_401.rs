// Generated macro for impl_401 (impl)
macro_rules! Depcrate_client_proxy_matcherimpl_401 {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"impl_401"}
// Dependencies: {}
impl fmt :: Debug for Matcher { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut b = f . debug_struct ("Matcher") ; if let Some (ref http) = self . http { b . field ("http" , http) ; } if let Some (ref https) = self . https { b . field ("https" , https) ; } if ! self . no . is_empty () { b . field ("no" , & self . no) ; } b . finish () } }
};
}
