// Generated macro for TarjanScc (struct)
macro_rules! Depcrate_algo_scc_tarjan_sccTarjanScc {
() => {
// Module: crate::algo::scc::tarjan_scc
// Provides: {"TarjanScc"}
// Dependencies: {}
# [doc = " A reusable state for computing the *strongly connected components* using [Tarjan's algorithm][1]."] # [doc = ""] # [doc = " [1]: https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm"] # [derive (Debug)] pub struct TarjanScc < N > { index : usize , componentcount : usize , nodes : Vec < NodeData > , stack : Vec < N > , }
};
}
