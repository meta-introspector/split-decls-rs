// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorsimpl_10 {
() => {
// Module: crate::errors
// Provides: {"impl_10"}
// Dependencies: {}
impl core :: error :: Error for Error { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: B64Encoding (err) => Some (err) , Self :: ParamValueInvalid (err) => Some (err) , Self :: SaltInvalid (err) => Some (err) , _ => None , } } }
};
}
