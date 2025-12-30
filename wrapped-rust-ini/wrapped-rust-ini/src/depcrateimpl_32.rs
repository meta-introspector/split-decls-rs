// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl DoubleEndedIterator for PropertiesIntoIter { # [cfg_attr (not (feature = "case-insensitive") , allow (clippy :: useless_conversion))] fn next_back (& mut self) -> Option < Self :: Item > { self . inner . next_back () . map (| (k , v) | (k . into () , v)) } }
};
}
