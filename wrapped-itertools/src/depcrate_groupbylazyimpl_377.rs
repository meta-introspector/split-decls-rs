// Generated macro for impl_377 (impl)
macro_rules! Depcrate_groupbylazyimpl_377 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_377"}
// Dependencies: {}
impl < 'a , I > IntoIterator for & 'a IntoChunks < I > where I : Iterator , I :: Item : 'a , { type Item = Chunk < 'a , I > ; type IntoIter = Chunks < 'a , I > ; fn into_iter (self) -> Self :: IntoIter { Chunks { parent : self } } }
};
}
