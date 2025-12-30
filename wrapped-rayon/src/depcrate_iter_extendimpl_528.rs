// Generated macro for impl_528 (impl)
macro_rules! Depcrate_iter_extendimpl_528 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_528"}
// Dependencies: {}
impl < T : Send > Consumer < T > for ListVecConsumer { type Folder = ListVecFolder < T > ; type Reducer = ListReducer ; type Result = LinkedList < Vec < T > > ; fn split_at (self , _index : usize) -> (Self , Self , Self :: Reducer) { (Self , Self , ListReducer) } fn into_folder (self) -> Self :: Folder { ListVecFolder { vec : Vec :: new () } } fn full (& self) -> bool { false } }
};
}
