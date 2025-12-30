// Generated macro for impl_546 (impl)
macro_rules! Depcrate_iter_extendimpl_546 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_546"}
// Dependencies: {}
impl < T : Send > Consumer < T > for ListConsumer { type Folder = ListFolder < T > ; type Reducer = ListReducer ; type Result = LinkedList < T > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListFolder { list : LinkedList :: new () , } } fn full (& self) -> bool { false } }
};
}
