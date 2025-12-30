// Generated macro for impl_1515 (impl)
macro_rules! Depcrate_stringimpl_1515 {
() => {
// Module: crate::string
// Provides: {"impl_1515"}
// Dependencies: {}
# [stable (feature = "drain" , since = "1.6.0")] impl Iterator for Drain < '_ > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . iter . next () } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn last (mut self) -> Option < char > { self . next_back () } }
};
}
