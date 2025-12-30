// Generated macro for impl_142 (impl)
macro_rules! Depcrate_encodeimpl_142 {
() => {
// Module: crate::encode
// Provides: {"impl_142"}
// Dependencies: {}
impl error :: Error for Error { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: InvalidValueWrite (ref err) => Some (err) , Self :: UnknownLength => None , Self :: InvalidDataModel (_) => None , Self :: DepthLimitExceeded => None , Self :: Syntax (..) => None , } } }
};
}
