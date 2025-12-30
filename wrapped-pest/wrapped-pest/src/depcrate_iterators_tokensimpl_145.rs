// Generated macro for impl_145 (impl)
macro_rules! Depcrate_iterators_tokensimpl_145 {
() => {
// Module: crate::iterators::tokens
// Provides: {"impl_145"}
// Dependencies: {}
impl < R : RuleType > DoubleEndedIterator for Tokens < '_ , R > { fn next_back (& mut self) -> Option < Self :: Item > { if self . end <= self . start { return None ; } let token = self . create_token (self . end - 1) ; self . end -= 1 ; Some (token) } }
};
}
