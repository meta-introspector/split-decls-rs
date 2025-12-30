// Generated macro for impl_558 (impl)
macro_rules! Depcrate_uriimpl_558 {
() => {
// Module: crate::uri
// Provides: {"impl_558"}
// Dependencies: {}
# [doc = " Convert an `Authority` into a `Uri`."] impl From < Authority > for Uri { fn from (authority : Authority) -> Self { Self { scheme : Scheme :: empty () , authority , path_and_query : PathAndQuery :: empty () , } } }
};
}
