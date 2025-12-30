// Generated macro for impl_94 (impl)
macro_rules! Depcrate_nodes_hamtimpl_94 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_94"}
// Dependencies: {}
impl < A > Drain < A > where A : HashValue , { pub (crate) fn new (pool : & Pool < Node < A > > , root : PoolRef < Node < A > > , size : usize) -> Self { Drain { count : size , pool : pool . clone () , stack : vec ! [] , current : root , collision : None , } } }
};
}
