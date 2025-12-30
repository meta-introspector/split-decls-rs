// Generated macro for impl_19 (impl)
macro_rules! Depcrate_errorimpl_19 {
() => {
// Module: crate::error
// Provides: {"impl_19"}
// Dependencies: {}
# [cfg (feature = "std")] impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self . kind { ErrorKind :: Adhoc (_) => None , ErrorKind :: Jiff (ref err) => Some (err) , } } }
};
}
