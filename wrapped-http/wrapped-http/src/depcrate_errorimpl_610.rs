// Generated macro for impl_610 (impl)
macro_rules! Depcrate_errorimpl_610 {
() => {
// Module: crate::error
// Provides: {"impl_610"}
// Dependencies: {}
impl Error { # [doc = " Return true if the underlying error has the same type as T."] pub fn is < T : error :: Error + 'static > (& self) -> bool { self . get_ref () . is :: < T > () } # [doc = " Return a reference to the lower level, inner error."] pub fn get_ref (& self) -> & (dyn error :: Error + 'static) { use self :: ErrorKind :: * ; match self . inner { StatusCode (ref e) => e , Method (ref e) => e , Uri (ref e) => e , UriParts (ref e) => e , HeaderName (ref e) => e , HeaderValue (ref e) => e , MaxSizeReached (ref e) => e , } } }
};
}
