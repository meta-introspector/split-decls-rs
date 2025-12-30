// Generated macro for impl_26 (impl)
macro_rules! Depcrate_fragmentsimpl_26 {
() => {
// Module: crate::fragments
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'sval > sval_ref :: ValueRef < 'sval > for TextBuf < 'sval > { fn stream_ref < S : sval :: Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> sval :: Result { match self . as_borrowed_str () { Some (v) => stream . value (v) , None => { let v = self . as_str () ; stream . text_begin (Some (v . len ())) ? ; stream . text_fragment_computed (v) ? ; stream . text_end () } } } }
};
}
