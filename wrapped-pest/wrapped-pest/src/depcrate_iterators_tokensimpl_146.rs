// Generated macro for impl_146 (impl)
macro_rules! Depcrate_iterators_tokensimpl_146 {
() => {
// Module: crate::iterators::tokens
// Provides: {"impl_146"}
// Dependencies: {}
impl < R : RuleType > fmt :: Debug for Tokens < '_ , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . clone ()) . finish () } }
};
}
