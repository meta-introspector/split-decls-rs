// Generated macro for impl_583 (impl)
macro_rules! Depcrate_provider_pattern_errorimpl_583 {
() => {
// Module: crate::provider::pattern::error
// Provides: {"impl_583"}
// Dependencies: {}
impl From < fields :: Error > for PatternError { fn from (input : fields :: Error) -> Self { match input { fields :: Error :: InvalidLength (symbol) => Self :: FieldLengthInvalid (symbol) , } } }
};
}
