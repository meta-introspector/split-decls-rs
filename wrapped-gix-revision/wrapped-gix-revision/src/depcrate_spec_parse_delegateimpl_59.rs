// Generated macro for impl_59 (impl)
macro_rules! Depcrate_spec_parse_delegateimpl_59 {
() => {
// Module: crate::spec::parse::delegate
// Provides: {"impl_59"}
// Dependencies: {}
impl SiblingBranch { # [doc = " Parse `input` as branch representation, if possible."] pub fn parse (input : & BStr) -> Option < Self > { if input . eq_ignore_ascii_case (b"u") || input . eq_ignore_ascii_case (b"upstream") { SiblingBranch :: Upstream . into () } else if input . eq_ignore_ascii_case (b"push") { SiblingBranch :: Push . into () } else { None } } }
};
}
