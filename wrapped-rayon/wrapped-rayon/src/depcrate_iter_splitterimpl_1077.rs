// Generated macro for impl_1077 (impl)
macro_rules! Depcrate_iter_splitterimpl_1077 {
() => {
// Module: crate::iter::splitter
// Provides: {"impl_1077"}
// Dependencies: {}
impl < 'a , D , S > UnindexedProducer for SplitProducer < 'a , D , S > where D : Send , S : Fn (D) -> (D , Option < D >) + Sync , { type Item = D ; fn split (mut self) -> (Self , Option < Self >) { let splitter = self . splitter ; let (left , right) = splitter (self . data) ; self . data = left ; (self , right . map (| data | SplitProducer { data , splitter })) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder . consume (self . data) } }
};
}
