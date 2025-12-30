// Generated macro for impl_58 (impl)
macro_rules! Depcrate_displayimpl_58 {
() => {
// Module: crate::display
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > fmt :: Display for AnsiString < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let w : & mut dyn fmt :: Write = f ; self . write_to_any (w) } }
};
}
