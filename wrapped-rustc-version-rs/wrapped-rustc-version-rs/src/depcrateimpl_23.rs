// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl error :: Error for LlvmVersionParseError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self { LlvmVersionParseError :: ParseIntError (e) => Some (e) , LlvmVersionParseError :: ComponentMustNotHaveLeadingZeros | LlvmVersionParseError :: ComponentMustNotHaveSign | LlvmVersionParseError :: MinorVersionMustBeZeroAfter4 | LlvmVersionParseError :: MinorVersionRequiredBefore4 | LlvmVersionParseError :: TooManyComponents => None , } } }
};
}
