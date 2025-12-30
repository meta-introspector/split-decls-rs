// Generated macro for impl_703 (impl)
macro_rules! Depcrate_iter_foldimpl_703 {
() => {
// Module: crate::iter::fold
// Provides: {"impl_703"}
// Dependencies: {}
impl < 'r , U , T , C , F > Consumer < T > for FoldWithConsumer < 'r , C , U , F > where C : Consumer < U > , F : Fn (U , T) -> U + Sync , U : Send + Clone , { type Folder = FoldFolder < 'r , C :: Folder , U , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (FoldWithConsumer { base : left , item : self . item . clone () , .. self } , FoldWithConsumer { base : right , .. self } , reducer ,) } fn into_folder (self) -> Self :: Folder { FoldFolder { base : self . base . into_folder () , item : self . item , fold_op : self . fold_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
