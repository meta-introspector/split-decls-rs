// Generated macro for impl_1159 (impl)
macro_rules! Depcrate_iter_try_foldimpl_1159 {
() => {
// Module: crate::iter::try_fold
// Provides: {"impl_1159"}
// Dependencies: {}
impl < 'r , U , T , C , F > Consumer < T > for TryFoldWithConsumer < 'r , C , U , F > where C : Consumer < U > , F : Fn (U :: Output , T) -> U + Sync , U : Try < Output : Clone + Send > + Send , { type Folder = TryFoldFolder < 'r , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (TryFoldWithConsumer { base : left , item : self . item . clone () , .. self } , TryFoldWithConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { TryFoldFolder { base : self . base . into_folder () , control : Continue (self . item) , fold_op : self . fold_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
