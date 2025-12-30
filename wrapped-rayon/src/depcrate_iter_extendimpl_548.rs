// Generated macro for impl_548 (impl)
macro_rules! Depcrate_iter_extendimpl_548 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_548"}
// Dependencies: {}
impl < T > Folder < T > for ListFolder < T > { type Result = LinkedList < T > ; fn consume (mut self , item : T) -> Self { self . list . push_back (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . list . extend (iter) ; self } fn complete (self) -> Self :: Result { self . list } fn full (& self) -> bool { false } }
};
}
