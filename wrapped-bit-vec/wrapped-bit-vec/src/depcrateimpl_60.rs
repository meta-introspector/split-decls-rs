// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < B : BitBlock > DoubleEndedIterator for Iter < '_ , B > { # [inline] fn next_back (& mut self) -> Option < bool > { self . range . next_back () . map (| i | self . bit_vec . get (i) . unwrap ()) } }
};
}
