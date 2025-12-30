// Generated macro for impl_9 (impl)
macro_rules! Depcrate_seqimpl_9 {
() => {
// Module: crate::seq
// Provides: {"impl_9"}
// Dependencies: {}
impl < 'sval , T : ValueRef < 'sval > > ValueRef < 'sval > for [T] { fn stream_ref < S : Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> Result { stream . seq_begin (Some (self . len ())) ? ; for elem in self { stream . seq_value_begin () ? ; crate :: stream_ref (stream , elem) ? ; stream . seq_value_end () ? ; } stream . seq_end () } }
};
}
