// Generated macro for impl_80 (impl)
macro_rules! Depcrate_nodes_hamtimpl_80 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_80"}
// Dependencies: {}
impl < A > Node < A > { # [inline] pub (crate) fn new () -> Self { Node { data : SparseChunk :: new () , } } # [inline] fn len (& self) -> usize { self . data . len () } # [inline] pub (crate) fn unit (index : usize , value : Entry < A >) -> Self { Node { data : SparseChunk :: unit (index , value) , } } # [inline] pub (crate) fn pair (index1 : usize , value1 : Entry < A > , index2 : usize , value2 : Entry < A >) -> Self { Node { data : SparseChunk :: pair (index1 , value1 , index2 , value2) , } } # [inline] pub (crate) fn single_child (pool : & Pool < Node < A > > , index : usize , node : Self) -> Self { Node { data : SparseChunk :: unit (index , Entry :: from_node (pool , node)) , } } fn pop (& mut self) -> Entry < A > { self . data . pop () . unwrap () } }
};
}
