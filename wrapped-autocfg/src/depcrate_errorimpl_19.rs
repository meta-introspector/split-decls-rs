// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
impl error :: Error for Error { fn description (& self) -> & str { "AutoCfg error" } fn cause (& self) -> Option < & error :: Error > { match self . kind { ErrorKind :: Io (ref e) => Some (e) , ErrorKind :: Num (ref e) => Some (e) , ErrorKind :: Utf8 (ref e) => Some (e) , ErrorKind :: Process (_) | ErrorKind :: Other (_) => None , } } }
};
}
