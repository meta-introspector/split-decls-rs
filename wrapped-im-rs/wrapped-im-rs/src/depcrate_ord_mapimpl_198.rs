// Generated macro for impl_198 (impl)
macro_rules! Depcrate_ord_mapimpl_198 {
() => {
// Module: crate::ord::map
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a , K , V > Iterator for Iter < 'a , K , V > where (K , V) : 'a + BTreeValue , { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (k , v) | (k , v)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . it . remaining , Some (self . it . remaining)) } }
};
}
