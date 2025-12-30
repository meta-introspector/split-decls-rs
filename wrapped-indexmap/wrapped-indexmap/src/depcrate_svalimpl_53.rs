// Generated macro for impl_53 (impl)
macro_rules! Depcrate_svalimpl_53 {
() => {
// Module: crate::sval
// Provides: {"impl_53"}
// Dependencies: {}
impl < K : Value , S > Value for IndexSet < K , S > { fn stream < 'sval , ST : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut ST) -> sval :: Result { stream . seq_begin (Some (self . len ())) ? ; for value in self { stream . seq_value_begin () ? ; stream . value (value) ? ; stream . seq_value_end () ? ; } stream . seq_end () } }
};
}
