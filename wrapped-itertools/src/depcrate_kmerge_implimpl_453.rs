// Generated macro for impl_453 (impl)
macro_rules! Depcrate_kmerge_implimpl_453 {
() => {
// Module: crate::kmerge_impl
// Provides: {"impl_453"}
// Dependencies: {}
impl < I , F > Iterator for KMergeBy < I , F > where I : Iterator , F : KMergePredicate < I :: Item > , { type Item = I :: Item ; fn next (& mut self) -> Option < Self :: Item > { if self . heap . is_empty () { return None ; } let result = if let Some (next) = self . heap [0] . next () { next } else { self . heap . swap_remove (0) . head } ; let less_than = & mut self . less_than ; sift_down (& mut self . heap , 0 , | a , b | { less_than . kmerge_pred (& a . head , & b . head) }) ; Some (result) } fn size_hint (& self) -> (usize , Option < usize >) { self . heap . iter () . map (| i | i . size_hint ()) . reduce (size_hint :: add) . unwrap_or ((0 , Some (0))) } }
};
}
