// Generated macro for impl_694 (impl)
macro_rules! Depcrate_iter_foldimpl_694 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_694"}
// Dependencies: {}
impl < 'r , U , T , C , ID , F > Consumer < T > for FoldConsumer < 'r , C , ID , F > where C : Consumer < U > , F : Fn (U , T) -> U + Sync , ID : Fn () -> U + Sync , U : Send , { type Folder = FoldFolder < 'r , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FoldConsumer { base : left , .. self } , FoldConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { FoldFolder { base : self . base . into_folder () , item : (self . identity) () , fold_op : self . fold_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
