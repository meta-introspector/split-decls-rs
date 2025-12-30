// Generated macro for impl_1641 (impl)
macro_rules! Depcrate_strimpl_1641 {
() => {
// Module: crate::str
// Provides: {"impl_1641"}
// Dependencies: {}
impl < 'ch > UnindexedProducer for BytesProducer < 'ch > { type Item = u8 ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (BytesProducer { chars : left } , Some (BytesProducer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . bytes ()) } }
};
}
