// Generated macro for impl_364 (impl)
macro_rules! Depcrate_groupbylazyimpl_364 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_364"}
// Dependencies: {}
impl < 'a , K , I , F > IntoIterator for & 'a ChunkBy < K , I , F > where I : Iterator , I :: Item : 'a , F : FnMut (& I :: Item) -> K , K : PartialEq , { type Item = (K , Group < 'a , K , I , F >) ; type IntoIter = Groups < 'a , K , I , F > ; fn into_iter (self) -> Self :: IntoIter { Groups { parent : self } } }
};
}
