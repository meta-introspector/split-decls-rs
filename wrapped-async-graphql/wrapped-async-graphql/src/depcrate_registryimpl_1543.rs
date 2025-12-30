// Generated macro for impl_1543 (impl)
macro_rules! Depcrate_registryimpl_1543 {
() => {
// Module: crate::registry
// Provides: {"impl_1543"}
// Dependencies: {}
impl Deprecation { # [inline] pub fn is_deprecated (& self) -> bool { matches ! (self , Deprecation :: Deprecated { .. }) } # [inline] pub fn reason (& self) -> Option < & str > { match self { Deprecation :: NoDeprecated => None , Deprecation :: Deprecated { reason } => reason . as_deref () , } } }
};
}
