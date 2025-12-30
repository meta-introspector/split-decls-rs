// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl Iterator for PropertiesIntoIter { type Item = (String , String) ; # [cfg_attr (not (feature = "case-insensitive") , allow (clippy :: useless_conversion))] fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . into () , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
