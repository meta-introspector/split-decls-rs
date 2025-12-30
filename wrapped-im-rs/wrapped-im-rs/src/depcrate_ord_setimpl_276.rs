// Generated macro for impl_276 (impl)
macro_rules! Depcrate_ord_setimpl_276 {
() => {
// Module: crate::ord::set
// Provides: {"impl_276"}
// Dependencies: {}
impl < 'a , A > Iterator for DiffIter < 'a , A > where A : Ord + PartialEq , { type Item = DiffItem < 'a , A > ; # [doc = " Advance the iterator and return the next value."] # [doc = ""] # [doc = " Time: O(1)*"] fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| item | match item { DiffItem :: Add (v) => DiffItem :: Add (v . deref ()) , DiffItem :: Update { old , new } => DiffItem :: Update { old : old . deref () , new : new . deref () , } , DiffItem :: Remove (v) => DiffItem :: Remove (v . deref ()) , }) } }
};
}
