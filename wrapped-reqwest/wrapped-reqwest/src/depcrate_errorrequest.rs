// Generated macro for request (function)
macro_rules! Depcrate_errorrequest {
() => {
// Module: crate::error
// Provides: {"request"}
// Dependencies: {}
pub (crate) fn request < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Request , Some (e)) }
};
}
