// Generated macro for impl_559 (impl)
macro_rules! Depcrate_iter_extendimpl_559 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_559"}
// Dependencies: {}
impl Folder < char > for ListStringFolder { type Result = LinkedList < String > ; fn consume (mut self , item : char) -> Self { self . string . push (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = char > , { self . string . extend (iter) ; self } fn complete (self) -> Self :: Result { let mut list = LinkedList :: new () ; if ! self . string . is_empty () { list . push_back (self . string) ; } list } fn full (& self) -> bool { false } }
};
}
