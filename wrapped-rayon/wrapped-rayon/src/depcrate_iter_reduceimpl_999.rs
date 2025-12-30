// Generated macro for impl_999 (impl)
macro_rules! Depcrate_iter_reduceimpl_999 {
() => {
// Module: crate::iter::reduce
// Provides: {"impl_999"}
// Dependencies: {}
impl < 'r , R , T > Folder < T > for ReduceFolder < 'r , R , T > where R : Fn (T , T) -> T , { type Result = T ; fn consume (self , item : T) -> Self { ReduceFolder { reduce_op : self . reduce_op , item : (self . reduce_op) (self . item , item) , } } fn consume_iter < I > (self , iter : I) -> Self where I : IntoIterator < Item = T > , { ReduceFolder { reduce_op : self . reduce_op , item : iter . into_iter () . fold (self . item , self . reduce_op) , } } fn complete (self) -> T { self . item } fn full (& self) -> bool { false } }
};
}
