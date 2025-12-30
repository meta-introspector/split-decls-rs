// Generated macro for impl_136 (impl)
macro_rules! Depcrateimpl_136 {
() => {
// Module: crate
// Provides: {"impl_136"}
// Dependencies: {}
impl < T > iter :: DoubleEndedIterator for Unwrap < T > where T : DoubleEndedFallibleIterator , T :: Error : core :: fmt :: Debug , { # [inline] fn next_back (& mut self) -> Option < T :: Item > { self . 0 . next_back () . unwrap () } }
};
}
