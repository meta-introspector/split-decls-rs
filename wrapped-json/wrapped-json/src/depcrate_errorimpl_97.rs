// Generated macro for impl_97 (impl)
macro_rules! Depcrate_errorimpl_97 {
() => {
// Module: crate::error
// Provides: {"impl_97"}
// Dependencies: {}
impl serde :: de :: StdError for Error { # [cfg (feature = "std")] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match & self . err . code { ErrorCode :: Io (err) => err . source () , _ => None , } } }
};
}
