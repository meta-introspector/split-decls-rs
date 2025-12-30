// Generated macro for impl_139 (impl)
macro_rules! Depcrateimpl_139 {
() => {
// Module: crate
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Intersection < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . rev () . find (| & nxt | self . other . contains (nxt)) } }
};
}
