// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let error = self . as_serde_de_error :: < serde :: de :: value :: Error > () ; Display :: fmt (& error , formatter) } }
};
}
