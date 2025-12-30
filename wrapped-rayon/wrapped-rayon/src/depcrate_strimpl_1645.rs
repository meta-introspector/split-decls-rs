// Generated macro for impl_1645 (impl)
macro_rules! Depcrate_strimpl_1645 {
() => {
// Module: crate::str
// Provides: {"impl_1645"}
// Dependencies: {}
impl < 'ch > UnindexedProducer for EncodeUtf16Producer < 'ch > { type Item = u16 ; fn split (self) -> (Self , Option < Self >) { match split (self . chars) { Some ((left , right)) => (EncodeUtf16Producer { chars : left } , Some (EncodeUtf16Producer { chars : right }) ,) , None => (self , None) , } } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume_iter (self . chars . encode_utf16 ()) } }
};
}
