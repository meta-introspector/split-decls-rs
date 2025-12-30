// Generated macro for impl_530 (impl)
macro_rules! Depcrate_iter_extendimpl_530 {
() => {
// Module: crate::iter::extend
// Provides: {"impl_530"}
// Dependencies: {}
impl < T > Folder < T > for ListVecFolder < T > { type Result = LinkedList < Vec < T > > ; fn consume (mut self , item : T) -> Self { self . vec . push (item) ; self } fn consume_iter < I > (mut self , iter : I) -> Self where I : IntoIterator < Item = T > , { self . vec . extend (iter) ; self } fn complete (self) -> Self :: Result { let mut list = LinkedList :: new () ; if ! self . vec . is_empty () { list . push_back (self . vec) ; } list } fn full (& self) -> bool { false } }
};
}
