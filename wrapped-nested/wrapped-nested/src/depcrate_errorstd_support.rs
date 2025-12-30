// Generated macro for std_support (module)
macro_rules! Depcrate_errorstd_support {
() => {
// Module: crate::error
// Provides: {"std_support"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_support { use super :: * ; use std :: error ; impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . 0 { ErrorKind :: Buffer (ref err) => Some (err) , ErrorKind :: InvalidValue { .. } => None , # [cfg (not (feature = "alloc"))] ErrorKind :: NoAlloc { .. } => None , } } } }
};
}
