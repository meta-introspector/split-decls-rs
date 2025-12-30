// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , K , V > Iterator for CLruCacheIter < 'a , K , V > { type Item = (& 'a K , & 'a V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (_ , CLruNode { key , value }) | (key . borrow () , value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
