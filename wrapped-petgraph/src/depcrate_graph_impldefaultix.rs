// Generated macro for DefaultIx (type)
macro_rules! Depcrate_graph_implDefaultIx {
() => {
// Module: crate::graph_impl
// Provides: {"DefaultIx"}
// Dependencies: {}
# [doc = " The default integer type for graph indices."] # [doc = " `u32` is the default to reduce the size of the graph's data and improve"] # [doc = " performance in the common case."] # [doc = ""] # [doc = " Used for node and edge indices in `Graph` and `StableGraph`, used"] # [doc = " for node indices in `Csr`."] pub type DefaultIx = u32 ;
};
}
