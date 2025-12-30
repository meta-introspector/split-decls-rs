// Generated macro for impl_575 (impl)
macro_rules! Depcrate_errorimpl_575 {
() => {
// Module: crate::error
// Provides: {"impl_575"}
// Dependencies: {}
impl core :: error :: Error for DecodeError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: Utf8 { inner } => Some (inner) , _ => None , } } }
};
}
