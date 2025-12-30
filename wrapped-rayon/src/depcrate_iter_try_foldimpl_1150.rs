// Generated macro for impl_1150 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1150 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1150"}
// Dependencies: {}
impl < 'r , U , T , C , ID , F > Consumer < T > for TryFoldConsumer < 'r , U , C , ID , F > where C : Consumer < U > , F : Fn (U :: Output , T) -> U + Sync , ID : Fn () -> U :: Output + Sync , U : Try + Send , { type Folder = TryFoldFolder < 'r , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TryFoldConsumer { base : left , .. self } , TryFoldConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TryFoldFolder { base : self . base . into_folder () , control : Continue ((self . identity) ()) , fold_op : self . fold_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
