// Generated macro for impl_42 (impl)
macro_rules! Depcrate_flatimpl_42 {
() => {
// Module: crate::flat
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > sval :: Value for Tag < 'a > { fn stream < 'sval , S : sval :: Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut S) -> sval :: Result { stream . tag (self . 0 , self . 1 , self . 2) } }
};
}
