// Generated macro for impl_1637 (impl)
macro_rules! Depcrate_strimpl_1637 {
() => {
// Module: crate::str
// Provides: {"impl_1637"}
// Dependencies: {}
impl < 'ch > UnindexedProducer for CharIndicesProducer < 'ch > { type Item = (usize , char) ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (CharIndicesProducer { chars : left , .. self } , Some (CharIndicesProducer { chars : right , index : self . index + left . len () , }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { let base = self . index ; folder . consume_iter (self . chars . char_indices () . map (offset (base))) } }
};
}
