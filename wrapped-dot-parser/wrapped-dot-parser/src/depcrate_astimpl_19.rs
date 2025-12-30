// Generated macro for impl_19 (impl)
macro_rules! Depcrate_astimpl_19 {
() => {
// Module: crate::ast
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > ParseError < 'a > { fn expect_rule (expect : Vec < Rule > , found : Rule) -> Self { ParseError :: ExpectRule { expect , found } } fn missing_pair (parent : Pair < 'a , Rule > , expect : Vec < Rule >) -> Self { ParseError :: MissingPair { parent , expect } } }
};
}
