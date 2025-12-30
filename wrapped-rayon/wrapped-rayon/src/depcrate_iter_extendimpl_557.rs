// Generated macro for impl_557 (impl)
macro_rules! Depcrate_iter_extendimpl_557 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_557"}
// Dependencies: {}
impl Consumer < char > for ListStringConsumer { type Folder = ListStringFolder ; type Reducer = ListReducer ; type Result = LinkedList < String > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListStringFolder { string : String :: new () , } } fn full (& self) -> bool { false } }
};
}
