// Generated macro for impl_130 (impl)
macro_rules! Depcrateimpl_130 {
() => {
// Module: crate
// Provides: {"impl_130"}
// Dependencies: {}
impl < I : Iterator , const N : usize > DoubleEndedIterator for Splice < '_ , I , N > { fn next_back (& mut self) -> Option < Self :: Item > { self . drain . next_back () } }
};
}
