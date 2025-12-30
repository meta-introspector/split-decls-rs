// Generated macro for impl_67 (impl)
macro_rules! Depcrateimpl_67 {
() => {
// Module: crate
// Provides: {"impl_67"}
// Dependencies: {}
impl < B : BitBlock > DoubleEndedIterator for IntoIter < B > { # [inline] fn next_back (& mut self) -> Option < bool > { self . range . next_back () . map (| i | self . bit_vec . get (i) . unwrap ()) } }
};
}
