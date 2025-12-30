// Generated macro for impl_52 (impl)
macro_rules! Depcrate_svalimpl_52 {
() => {
// Module: crate::sval
// Provides: {"impl_52"}
// Dependencies: {}
impl < K : Value , V : Value , S > Value for IndexMap < K , V , S > { fn stream < 'sval , ST : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut ST) -> sval :: Result { stream . map_begin (Some (self . len ())) ? ; for (k , v) in self { stream . map_key_begin () ? ; stream . value (k) ? ; stream . map_key_end () ? ; stream . map_value_begin () ? ; stream . value (v) ? ; stream . map_value_end () ? ; } stream . map_end () } }
};
}
