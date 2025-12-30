// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , K , V > Iterator for CLruCacheIterMut < 'a , K , V > { type Item = (& 'a K , & 'a mut V) ; fn next (& mut self) -> Option < Self :: Item > { self . iter . next () . map (| (_ , CLruNode { key , value }) | (& * key , value)) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
