// Generated macro for body (function)
macro_rules! Depcrate_errorbody {
() => {
// Module: crate::error
// Provides: {"body"}
// Dependencies: {}
pub (crate) fn body < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Body , Some (e)) }
};
}
