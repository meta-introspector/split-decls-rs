// Generated macro for impl_19 (impl)
macro_rules! Depcrate_availabilityimpl_19 {
() => {
// Module: crate::availability
// Provides: {"impl_19"}
// Dependencies: {}
impl fmt :: Display for Availability { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . deprecated { _ if ! self . is_deprecated () => { } Versions { .. } => { if let Some (message) = & self . message { writeln ! (f , "#[deprecated = {message:?}]") ? ; } else { writeln ! (f , "#[deprecated]") ? ; } } } Ok (()) } }
};
}
