// Generated macro for impl_559 (impl)
macro_rules! Depcrate_uriimpl_559 {
() => {
// Module: crate::uri
// Provides: {"impl_559"}
// Dependencies: {}
# [doc = " Convert a `PathAndQuery` into a `Uri`."] impl From < PathAndQuery > for Uri { fn from (path_and_query : PathAndQuery) -> Self { Self { scheme : Scheme :: empty () , authority : Authority :: empty () , path_and_query , } } }
};
}
