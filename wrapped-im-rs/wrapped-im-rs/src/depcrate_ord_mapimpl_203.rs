// Generated macro for impl_203 (impl)
macro_rules! Depcrate_ord_mapimpl_203 {
() => {
// Module: crate::ord::map
// Provides: {"impl_203"}
// Dependencies: {}
impl < 'a , K , V > Iterator for DiffIter < 'a , K , V > where (K , V) : 'a + BTreeValue + PartialEq , { type Item = DiffItem < 'a , K , V > ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| item | match item { NodeDiffItem :: Add ((k , v)) => DiffItem :: Add (k , v) , NodeDiffItem :: Update { old : (oldk , oldv) , new : (newk , newv) , } => DiffItem :: Update { old : (oldk , oldv) , new : (newk , newv) , } , NodeDiffItem :: Remove ((k , v)) => DiffItem :: Remove (k , v) , }) } }
};
}
