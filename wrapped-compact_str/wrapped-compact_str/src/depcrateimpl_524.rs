// Generated macro for impl_524 (impl)
macro_rules! Depcrateimpl_524 {
() => {
// Module: crate
// Provides: {"impl_524"}
// Dependencies: {}
impl Iterator for Drain < '_ > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { self . chars . next () } # [inline] fn count (self) -> usize { self . chars . clone () . count () } fn size_hint (& self) -> (usize , Option < usize >) { self . chars . size_hint () } # [inline] fn last (mut self) -> Option < char > { self . chars . next_back () } }
};
}
