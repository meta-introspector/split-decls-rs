// Generated macro for impl_8 (impl)
macro_rules! Depcrate_errorimpl_8 {
() => {
// Module: crate::error
// Provides: {"impl_8"}
// Dependencies: {}
impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Error :: Asn1 (err) => Some (err) , _ => None , } } }
};
}
