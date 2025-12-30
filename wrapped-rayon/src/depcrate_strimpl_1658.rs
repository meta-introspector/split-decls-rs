// Generated macro for impl_1658 (impl)
macro_rules! Depcrate_strimpl_1658 {
() => {
// Module: crate::str
// Provides: {"impl_1658"}
// Dependencies: {}
impl < 'ch , 'sep , P : Pattern + 'sep > UnindexedProducer for SplitTerminatorProducer < 'ch , 'sep , P > { type Item = & 'ch str ; fn split (mut self) -> (Self , Option < Self >) { let (left , right) = self . splitter . split () ; self . splitter = left ; let right = right . map (| right | { let skip_last = self . skip_last ; self . skip_last = false ; SplitTerminatorProducer { splitter : right , skip_last , } }) ; (self , right) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { self . splitter . fold_with (folder , self . skip_last) } }
};
}
