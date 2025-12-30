// Generated macro for impl_31 (impl)
macro_rules! Depcrate_to_valueimpl_31 {
() => {
// Module: crate::to_value
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : fmt :: Debug > sval :: Value for DebugToValue < T > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { struct Writer < S > (S) ; impl < 'a , S : sval :: Stream < 'a > > fmt :: Write for Writer < S > { fn write_str (& mut self , s : & str) -> fmt :: Result { self . 0 . text_fragment_computed (s) . map_err (| _ | fmt :: Error) ? ; Ok (()) } } stream . text_begin (None) ? ; write ! (Writer (& mut * stream) , "{:?}" , self . 0) . map_err (| _ | sval :: Error :: new ()) ? ; stream . text_end () } }
};
}
