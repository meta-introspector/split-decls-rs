// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > DoubleEndedIterator for IntoIter < T , CAP , B > { # [inline] fn next_back (& mut self) -> Option < T > { self . inner . pop_back () } }
};
}
