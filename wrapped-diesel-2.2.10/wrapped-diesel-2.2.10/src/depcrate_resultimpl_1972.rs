// Generated macro for impl_1972 (impl)
macro_rules! Depcrate_resultimpl_1972 {
() => {
// Module: crate::result
// Provides: {"impl_1972"}
// Dependencies: {}
impl Display for ConnectionError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { ConnectionError :: InvalidCString (ref nul_err) => nul_err . fmt (f) , ConnectionError :: BadConnection (ref s) => write ! (f , "{s}") , ConnectionError :: InvalidConnectionUrl (ref s) => write ! (f , "{s}") , ConnectionError :: CouldntSetupConfiguration (ref e) => e . fmt (f) , } } }
};
}
