// Generated macro for impl_89 (impl)
macro_rules! Depcrate_nodes_hamtimpl_89 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_89"}
// Dependencies: {}
impl < 'a , A > IterMut < 'a , A > where A : 'a , { pub (crate) fn new (pool : & Pool < Node < A > > , root : & 'a mut Node < A > , size : usize) -> Self { IterMut { count : size , pool : pool . clone () , stack : Vec :: with_capacity ((HASH_WIDTH / HASH_SHIFT) + 1) , current : root . data . iter_mut () , collision : None , } } }
};
}
