// Generated macro for impl_74 (impl)
macro_rules! Depcrate_provider_exception_helpersimpl_74 {
() => {
// Module: crate::provider::exception_helpers
// Provides: {"impl_74"}
// Dependencies: {}
impl From < MappingKind > for ExceptionSlot { fn from (full : MappingKind) -> Self { match full { MappingKind :: Lower => Self :: Lower , MappingKind :: Fold => Self :: Fold , MappingKind :: Upper => Self :: Upper , MappingKind :: Title => Self :: Title , } } }
};
}
