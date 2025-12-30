// Generated macro for decode (function)
macro_rules! Depcrate_errordecode {
() => {
// Module: crate::error
// Provides: {"decode"}
// Dependencies: {}
pub (crate) fn decode < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Decode , Some (e)) }
};
}
