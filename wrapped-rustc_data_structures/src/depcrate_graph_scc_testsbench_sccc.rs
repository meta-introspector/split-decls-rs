// Generated macro for bench_sccc (function)
macro_rules! Depcrate_graph_scc_testsbench_sccc {
() => {
// Module: crate::graph::scc::tests
// Provides: {"bench_sccc"}
// Dependencies: {}
# [bench] fn bench_sccc (b : & mut test :: Bencher) { fn make_3_clique (slice : & mut [(usize , usize)] , base : usize) { slice [0] = (base + 0 , base + 1) ; slice [1] = (base + 1 , base + 2) ; slice [2] = (base + 2 , base + 0) ; } fn make_4_clique (slice : & mut [(usize , usize)] , base : usize) { slice [0] = (base + 0 , base + 1) ; slice [1] = (base + 1 , base + 2) ; slice [2] = (base + 2 , base + 3) ; slice [3] = (base + 3 , base + 0) ; slice [4] = (base + 1 , base + 3) ; slice [5] = (base + 2 , base + 1) ; } let mut graph = [(0 , 0) ; 6 + 3 + 6 + 3 + 4] ; make_4_clique (& mut graph [0 .. 6] , 0) ; make_3_clique (& mut graph [6 .. 9] , 4) ; make_4_clique (& mut graph [9 .. 15] , 7) ; make_3_clique (& mut graph [15 .. 18] , 11) ; graph [18] = (0 , 4) ; graph [19] = (5 , 7) ; graph [20] = (11 , 10) ; graph [21] = (7 , 4) ; let graph = TestGraph :: new (0 , & graph [..]) ; b . iter (| | { let sccs : UsizeSccs = Sccs :: new (& graph) ; assert_eq ! (sccs . num_sccs () , 3) ; }) ; }
};
}
