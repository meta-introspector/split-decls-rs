// Generated macro for impl_84 (impl)
macro_rules! Depcrate_nodes_hamtimpl_84 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , A > Iter < 'a , A > where A : 'a , { pub (crate) fn new (root : & 'a Node < A > , size : usize) -> Self { Iter { count : size , stack : Vec :: with_capacity ((HASH_WIDTH / HASH_SHIFT) + 1) , current : root . data . iter () , collision : None , } } }
};
}
