// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl < B : BitBlock > Iterator for IntoIter < B > { type Item = bool ; # [inline] fn next (& mut self) -> Option < bool > { self . range . next () . map (| i | self . bit_vec . get (i) . unwrap ()) } }
};
}
