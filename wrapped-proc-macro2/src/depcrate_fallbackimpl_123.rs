// Generated macro for impl_123 (impl)
macro_rules! Depcrate_fallbackimpl_123 {
() => {
// Module: crate::fallback
// Provides: {"impl_123"}
// Dependencies: {}
impl TokenStreamBuilder { pub (crate) fn new () -> Self { TokenStreamBuilder { inner : RcVecBuilder :: new () , } } pub (crate) fn with_capacity (cap : usize) -> Self { TokenStreamBuilder { inner : RcVecBuilder :: with_capacity (cap) , } } pub (crate) fn push_token_from_parser (& mut self , tt : TokenTree) { self . inner . push (tt) ; } pub (crate) fn build (self) -> TokenStream { TokenStream { inner : self . inner . build () , } } }
};
}
