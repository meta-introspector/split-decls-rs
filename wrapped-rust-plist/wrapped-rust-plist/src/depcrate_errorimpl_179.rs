// Generated macro for impl_179 (impl)
macro_rules! Depcrate_errorimpl_179 {
() => {
// Module: crate::error
// Provides: {"impl_179"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match & self . inner . kind { ErrorKind :: Io (err) => Some (err) , _ => None , } } }
};
}
