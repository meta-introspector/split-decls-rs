// Generated macro for impl_131 (impl)
macro_rules! Depcrateimpl_131 {
() => {
// Module: crate
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Difference < 'a > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . by_ref () . rev () . find (| & nxt | ! self . other . contains (nxt)) } }
};
}
