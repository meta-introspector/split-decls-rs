// Generated macro for provide (function)
macro_rules! Depcrate_nightlyprovide {
() => {
// Module: crate::nightly
// Provides: {"provide"}
// Dependencies: {}
pub fn provide < 'a > (err : & 'a (impl Error + ? Sized) , request : & mut Request < 'a >) { Error :: provide (err , request) ; }
};
}
