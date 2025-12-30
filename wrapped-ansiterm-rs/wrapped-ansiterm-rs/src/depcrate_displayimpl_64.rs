// Generated macro for impl_64 (impl)
macro_rules! Depcrate_displayimpl_64 {
() => {
// Module: crate::display
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > fmt :: Display for ANSIStrings < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let f : & mut dyn fmt :: Write = f ; self . write_to_any (f) } }
};
}
