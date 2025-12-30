// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl < B : BitBlock > DoubleEndedIterator for Blocks < '_ , B > { # [inline] fn next_back (& mut self) -> Option < B > { self . iter . next_back () . cloned () } }
};
}
