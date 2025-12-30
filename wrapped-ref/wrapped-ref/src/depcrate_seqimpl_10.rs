// Generated macro for impl_10 (impl)
macro_rules! Depcrate_seqimpl_10 {
() => {
// Module: crate::seq
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'sval , T : ValueRef < 'sval > , const N : usize > ValueRef < 'sval > for [T ; N] { fn stream_ref < S : Stream < 'sval > + ? Sized > (& self , stream : & mut S) -> Result { stream . tagged_begin (Some (& tags :: CONSTANT_SIZE) , None , None) ? ; stream . seq_begin (Some (self . len ())) ? ; for elem in self { stream . seq_value_begin () ? ; crate :: stream_ref (stream , elem) ? ; stream . seq_value_end () ? ; } stream . seq_end () ? ; stream . tagged_end (Some (& tags :: CONSTANT_SIZE) , None , None) } }
};
}
