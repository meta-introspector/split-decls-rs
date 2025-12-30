// Generated macro for impl_669 (impl)
macro_rules! Depcrate_iter_flattenimpl_669 {
() => {
// Module: crate::iter::flatten
// Provides: {"impl_669"}
// Dependencies: {}
impl < T , C > Consumer < T > for FlattenConsumer < C > where C : UnindexedConsumer < T :: Item > , T : IntoParallelIterator , { type Folder = FlattenFolder < C , C :: Result > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , C :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FlattenConsumer :: new (left) , FlattenConsumer :: new (right) , reducer ,) } fn into_folder (self) -> Self :: Folder { FlattenFolder { base : self . base , previous : None , } } fn full (& self) -> bool { self . base . full () } }
};
}
