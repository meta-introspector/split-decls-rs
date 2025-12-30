// Generated macro for Drain (struct)
macro_rules! Depcrate_nodes_hamtDrain {
() => {
// Module: crate::nodes::hamt
// Provides: {"Drain"}
// Dependencies: {}
pub (crate) struct Drain < A > where A : HashValue , { count : usize , pool : Pool < Node < A > > , stack : Vec < PoolRef < Node < A > > > , current : PoolRef < Node < A > > , collision : Option < CollisionNode < A > > , }
};
}
