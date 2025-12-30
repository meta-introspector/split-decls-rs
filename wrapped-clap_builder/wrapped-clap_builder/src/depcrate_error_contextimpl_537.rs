// Generated macro for impl_537 (impl)
macro_rules! Depcrate_error_contextimpl_537 {
() => {
// Module: crate::error::context
// Provides: {"impl_537"}
// Dependencies: {}
impl ContextKind { # [doc = " End-user description of the error case, where relevant"] pub fn as_str (self) -> Option < & 'static str > { match self { Self :: InvalidSubcommand => Some ("Invalid Subcommand") , Self :: InvalidArg => Some ("Invalid Argument") , Self :: PriorArg => Some ("Prior Argument") , Self :: ValidSubcommand => Some ("Valid Subcommand") , Self :: ValidValue => Some ("Valid Value") , Self :: InvalidValue => Some ("Invalid Value") , Self :: ActualNumValues => Some ("Actual Number of Values") , Self :: ExpectedNumValues => Some ("Expected Number of Values") , Self :: MinValues => Some ("Minimum Number of Values") , Self :: SuggestedCommand => Some ("Suggested Command") , Self :: SuggestedSubcommand => Some ("Suggested Subcommand") , Self :: SuggestedArg => Some ("Suggested Argument") , Self :: SuggestedValue => Some ("Suggested Value") , Self :: TrailingArg => Some ("Trailing Argument") , Self :: Suggested => Some ("Suggested") , Self :: Usage => None , Self :: Custom => None , } } }
};
}
