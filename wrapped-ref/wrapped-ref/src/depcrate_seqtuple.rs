// Generated macro for tuple (macro)
macro_rules! Depcrate_seqtuple {
() => {
// Module: crate::seq
// Provides: {"tuple"}
// Dependencies: {}
macro_rules ! tuple { ($ ($ len : expr => ($ (self .$ i : tt : $ ty : ident ,) +) ,) +) => { $ (impl <'sval , $ ($ ty : ValueRef <'sval >) ,+> ValueRef <'sval > for ($ ($ ty ,) +) { fn stream_ref < S : Stream <'sval > + ? Sized > (& self , stream : & mut S) -> Result { stream . tuple_begin (None , None , None , Some ($ len)) ?; $ (stream . tuple_value_begin (None , & Index :: new ($ i) . with_tag (& sval :: tags :: VALUE_OFFSET)) ?; crate :: stream_ref (stream , & self .$ i) ?; stream . tuple_value_end (None , & Index :: new ($ i) . with_tag (& sval :: tags :: VALUE_OFFSET)) ?;) + stream . tuple_end (None , None , None) } }) + } }
};
}
