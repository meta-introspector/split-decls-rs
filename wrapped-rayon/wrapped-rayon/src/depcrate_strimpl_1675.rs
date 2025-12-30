// Generated macro for impl_1675 (impl)
macro_rules! Depcrate_strimpl_1675 {
() => {
// Module: crate::str
// Provides: {"impl_1675"}
// Dependencies: {}
impl < 'ch , 'pat , P : Pattern > UnindexedProducer for MatchIndicesProducer < 'ch , 'pat , P > { type Item = (usize , & 'ch str) ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (MatchIndicesProducer { chars : left , .. self } , Some (MatchIndicesProducer { chars : right , index : self . index + left . len () , .. self }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . pattern . fold_match_indices (self . chars , folder , self . index) } }
};
}
