// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'a > Iterator for Difference < 'a > { type Item = usize ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . find (| & nxt | ! self . other . contains (nxt)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
