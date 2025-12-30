// Generated macro for impl_329 (impl)
macro_rules! Depcrate_streamimpl_329 {
() => {
// Module: crate::stream
// Provides: {"impl_329"}
// Dependencies: {}
impl < Input > IteratorStream < Input > where Input : Iterator , { # [doc = " Converts an `Iterator` into a stream."] # [doc = ""] # [doc = " NOTE: This type do not implement `Positioned` and `Clone` and must be wrapped with types"] # [doc = "     such as `BufferedStreamRef` and `State` to become a `Stream` which can be parsed"] pub fn new < T > (iter : T) -> IteratorStream < Input > where T : IntoIterator < IntoIter = Input , Item = Input :: Item > , { IteratorStream (iter . into_iter ()) } }
};
}
