// Generated macro for std_support (module)
macro_rules! Depcrate_errorstd_support {
() => {
// Module: crate::error
// Provides: {"std_support"}
// Dependencies: {}
# [cfg (feature = "std")] mod std_support { use super :: * ; use std :: error ; impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . kind { ErrorKind :: IO (ref err) => Some (err) , _ => None , } } } }
};
}
