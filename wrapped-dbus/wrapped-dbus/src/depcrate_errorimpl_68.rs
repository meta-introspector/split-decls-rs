// Generated macro for impl_68 (impl)
macro_rules! Depcrate_errorimpl_68 {
() => {
// Module: crate::error
// Provides: {"impl_68"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { write ! (f , "D-Bus error: {} ({})" , self . message () . unwrap_or ("") , self . name () . unwrap_or ("")) } }
};
}
