// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_iter_updateimpl_1240 {
() => {
// Module: crate::iter::update
// Provides: {"impl_1240"}
// Dependencies: {}
impl < 'f , T , C , F > Consumer < T > for UpdateConsumer < 'f , C , F > where C : Consumer < T > , F : Fn (& mut T) + Send + Sync , { type Folder = UpdateFolder < 'f , C :: Folder , F > ; type Reducer = C :: Reducer ; type Result = C :: Result ; fn split_at (self , index : usize) -> (Self , Self , Self :: Reducer) { let (left , right , reducer) = self . base . split_at (index) ; (UpdateConsumer :: new (left , self . update_op) , UpdateConsumer :: new (right , self . update_op) , reducer ,) } fn into_folder (self) -> Self :: Folder { UpdateFolder { base : self . base . into_folder () , update_op : self . update_op , } } fn full (& self) -> bool { self . base . full () } }
};
}
