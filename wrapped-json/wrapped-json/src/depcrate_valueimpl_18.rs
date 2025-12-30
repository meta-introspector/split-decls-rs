// Generated macro for impl_18 (impl)
macro_rules! Depcrate_valueimpl_18 {
() => {
// Module: crate::value
// Provides: {"impl_18"}
// Dependencies: {}
impl sval :: Value for JsonStr { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { stream . tagged_begin (Some (& crate :: tags :: JSON_VALUE) , None , None) ? ; stream . value (& self . 0) ? ; stream . tagged_end (Some (& crate :: tags :: JSON_VALUE) , None , None) } }
};
}
