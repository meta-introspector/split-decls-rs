// Generated macro for impl_2691 (impl)
macro_rules! Depcrate_isaimpl_2691 {
() => {
// Module: crate::isa
// Provides: {"impl_2691"}
// Dependencies: {}
impl fmt :: Display for LookupError { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { match self { LookupError :: SupportDisabled => write ! (f , "Support for this target is disabled") , LookupError :: Unsupported => { write ! (f , "Support for this target has not been implemented yet") } } } }
};
}
