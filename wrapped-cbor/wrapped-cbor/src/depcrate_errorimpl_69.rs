// Generated macro for impl_69 (impl)
macro_rules! Depcrate_errorimpl_69 {
() => {
// Module: crate::error
// Provides: {"impl_69"}
// Dependencies: {}
# [cfg (feature = "std")] impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . 0 . code { ErrorCode :: Io (ref err) => Some (err) , _ => None , } } }
};
}
