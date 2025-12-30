// Generated macro for impl_497 (impl)
macro_rules! Depcrate_iter_emptyimpl_497 {
() => {
// Module: crate::iter::empty
// Provides: {"impl_497"}
// Dependencies: {}
impl < T : Send > Producer for EmptyProducer < T > { type Item = T ; type IntoIter = std :: iter :: Empty < T > ; fn into_iter (self) -> Self :: IntoIter { std :: iter :: empty () } fn split_at (self , index : usize) -> (Self , Self) { debug_assert_eq ! (index , 0) ; (self , EmptyProducer (PhantomData)) } fn fold_with < F > (self , folder : F) -> F where F : Folder < Self :: Item > , { folder } }
};
}
