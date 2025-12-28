macro_rules! deps {
    () => {
        DfsPostOrder!();
    };
}

macro_rules! Topo {
    () => {
        deps!();
        # [doc = " A topological order traversal for a graph."] # [doc = ""] # [doc = " **Note** that `Topo` only visits nodes that are not part of cycles,"] # [doc = " i.e. nodes in a true DAG. Use other visitors like [`DfsPostOrder`] or"] # [doc = " algorithms like [`kosaraju_scc`][crate::algo::kosaraju_scc()] to handle"] # [doc = " graphs with possible cycles."] # [derive (Clone)] pub struct Topo < N , VM > { tovisit : Vec < N > , ordered : VM , }
    };
}

Topo!();