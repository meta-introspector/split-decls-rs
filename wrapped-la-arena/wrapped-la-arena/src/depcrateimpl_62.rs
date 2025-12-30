// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = (Idx < T > , T) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . next () . map (| (idx , value) | (Idx :: from_raw (RawIdx (idx as u32)) , value)) } }
};
}
