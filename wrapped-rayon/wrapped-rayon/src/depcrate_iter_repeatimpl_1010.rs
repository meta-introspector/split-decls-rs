// Generated macro for impl_1010 (impl)
macro_rules! Depcrate_iter_repeatimpl_1010 {
() => {
// Module: crate::iter::repeat
// Provides: {"impl_1010"}
// Dependencies: {}
impl < T : Clone + Send > UnindexedProducer for RepeatProducer < T > { type Item = T ; fn split (self) -> (Self , Option < Self >) { (RepeatProducer { element : self . element . clone () , } , Some (RepeatProducer { element : self . element , }) ,) } fn fold_with < F > (self , folder : F) -> F where F : Folder < T > , { folder . consume_iter (iter :: repeat (self . element)) } }
};
}
