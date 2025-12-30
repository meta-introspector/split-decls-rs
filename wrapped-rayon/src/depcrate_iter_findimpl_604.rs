// Generated macro for impl_604 (impl)
macro_rules! Depcrate_iter_findimpl_604 {
() => {
// Module: crate::iter::find
// Provides: {"impl_604"}
// Dependencies: {}
impl < 'p , T , P : 'p > Consumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { type Folder = FindFolder < 'p , T , P > ; type Reducer = FindReducer ; type Result = Option < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (self . split_off_left () , self , FindReducer) } fn into_folder (self) -> Self :: Folder { FindFolder { find_op : self . find_op , found : self . found , item : None , } } fn full (& self) -> bool { self . found . load (Ordering :: Relaxed) } }
};
}
