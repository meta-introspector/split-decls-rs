// Generated macro for builder (function)
macro_rules! Depcrate_errorbuilder {
() => {
// Module: crate::error
// Provides: {"builder"}
// Dependencies: {}
pub (crate) fn builder < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Builder , Some (e)) }
};
}
