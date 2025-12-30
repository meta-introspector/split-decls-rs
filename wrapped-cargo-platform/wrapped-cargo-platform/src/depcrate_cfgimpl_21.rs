// Generated macro for impl_21 (impl)
macro_rules! Depcrate_cfgimpl_21 {
() => {
// Module: crate::cfg
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for Ident { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . raw { f . write_str ("r#") ? ; } f . write_str (& * self . name) } }
};
}
