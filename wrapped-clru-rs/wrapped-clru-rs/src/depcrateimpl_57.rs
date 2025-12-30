// Generated macro for impl_57 (impl)
macro_rules! Depcrateimpl_57 {
() => {
// Module: crate
// Provides: {"impl_57"}
// Dependencies: {}
impl < K , V > DoubleEndedIterator for CLruCacheIterMut < '_ , K , V > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (_ , CLruNode { key , value }) | (& * key , value)) } }
};
}
