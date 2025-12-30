// Generated macro for impl_161 (impl)
macro_rules! Depcrate_fallbackimpl_161 {
() => {
// Module: crate::fallback
// Provides: {"impl_161"}
// Dependencies: {}
impl Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . raw { f . write_str ("r#") ? ; } f . write_str (& self . sym) } }
};
}
