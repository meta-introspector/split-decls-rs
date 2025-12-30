// Generated macro for impl_61 (impl)
macro_rules! Depcrate_displayimpl_61 {
() => {
// Module: crate::display
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > fmt :: Display for AnsiStrings < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let f : & mut dyn fmt :: Write = f ; self . write_to_any (f) } }
};
}
