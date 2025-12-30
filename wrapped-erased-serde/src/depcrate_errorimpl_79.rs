// Generated macro for impl_79 (impl)
macro_rules! Depcrate_errorimpl_79 {
() => {
// Module: crate::error
// Provides: {"impl_79"}
// Dependencies: {}
impl Debug for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let error = self . as_serde_de_error :: < serde :: de :: value :: Error > () ; Debug :: fmt (& error , formatter) } }
};
}
