// Generated macro for impl_506 (impl)
macro_rules! Depcrate_multipeek_implimpl_506 {
() => {
// Module: crate::multipeek_impl
// Provides: {"impl_506"}
// Dependencies: {}
impl < I : Iterator > MultiPeek < I > { # [doc = " Works exactly like `.next()` with the only difference that it doesn't"] # [doc = " advance itself. `.peek()` can be called multiple times, to peek"] # [doc = " further ahead."] # [doc = " When `.next()` is called, reset the peeking “cursor”."] pub fn peek (& mut self) -> Option < & I :: Item > { let ret = if self . index < self . buf . len () { Some (& self . buf [self . index]) } else { match self . iter . next () { Some (x) => { self . buf . push_back (x) ; Some (& self . buf [self . index]) } None => return None , } } ; self . index += 1 ; ret } }
};
}
