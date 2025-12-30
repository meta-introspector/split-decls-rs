// Generated macro for PoolIndex (trait)
macro_rules! Depcrate_combinationsPoolIndex {
() => {
// Module: crate::combinations
// Provides: {"PoolIndex"}
// Dependencies: {}
# [doc = " A type holding indices of elements in a pool or buffer of items from an inner iterator"] # [doc = " and used to pick out different combinations in a generic way."] pub trait PoolIndex < T > : BorrowMut < [usize] > { type Item ; fn extract_item < I : Iterator < Item = T > > (& self , pool : & LazyBuffer < I >) -> Self :: Item where T : Clone ; fn len (& self) -> usize { self . borrow () . len () } }
};
}
