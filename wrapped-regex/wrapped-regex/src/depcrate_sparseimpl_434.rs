// Generated macro for impl_434 (impl)
macro_rules! Depcrate_sparseimpl_434 {
() => {
// Module: crate::sparse
// Provides: {"impl_434"}
// Dependencies: {}
impl SparseSet { pub fn new (size : usize) -> SparseSet { SparseSet { dense : vec ! [0 ; size] , sparse : vec ! [0 ; size] , size : 0 , } } pub fn len (& self) -> usize { self . size } pub fn is_empty (& self) -> bool { self . size == 0 } pub fn capacity (& self) -> usize { self . dense . len () } pub fn insert (& mut self , value : usize) { let i = self . size ; self . dense [i] = value ; self . sparse [value] = i ; self . size += 1 ; } pub fn contains (& self , value : usize) -> bool { let i = self . sparse [value] ; i < self . size && self . dense [i] == value } pub fn clear (& mut self) { self . size = 0 ; } }
};
}
