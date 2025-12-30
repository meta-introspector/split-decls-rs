// Generated macro for impl_52 (impl)
macro_rules! Depcrateimpl_52 {
() => {
// Module: crate
// Provides: {"impl_52"}
// Dependencies: {}
impl < K , V > DoubleEndedIterator for CLruCacheIter < '_ , K , V > { fn next_back (& mut self) -> Option < Self :: Item > { self . iter . next_back () . map (| (_ , CLruNode { key , value }) | (key . borrow () , value)) } }
};
}
