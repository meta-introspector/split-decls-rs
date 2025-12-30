// Generated macro for RRBPool (struct)
macro_rules! Depcrate_vector_poolRRBPool {
() => {
// Module: crate::vector::pool
// Provides: {"RRBPool"}
// Dependencies: {}
# [doc = " A memory pool for `Vector`."] pub struct RRBPool < A > { pub (crate) node_pool : Pool < Chunk < Node < A > > > , pub (crate) value_pool : Pool < Chunk < A > > , pub (crate) size_pool : Pool < Chunk < usize > > , }
};
}
