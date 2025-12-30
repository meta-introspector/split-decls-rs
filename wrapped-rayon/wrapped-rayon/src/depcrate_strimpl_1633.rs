// Generated macro for impl_1633 (impl)
macro_rules! Depcrate_strimpl_1633 {
() => {
// Module: crate::str
// Provides: {"impl_1633"}
// Dependencies: {}
impl < 'ch > UnindexedProducer for CharsProducer < 'ch > { type Item = char ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (CharsProducer { chars : left } , Some (CharsProducer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . chars ()) } }
};
}
