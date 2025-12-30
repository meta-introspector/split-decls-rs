// Generated macro for impl_544 (impl)
macro_rules! Depcrate_tokenstreamimpl_544 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_544"}
// Dependencies: {}
impl < 't > TokenStreamIter < 't > { fn new (stream : & 't TokenStream) -> Self { TokenStreamIter { stream , index : 0 } } pub fn peek (& self) -> Option < & 't TokenTree > { self . stream . 0 . get (self . index) } }
};
}
