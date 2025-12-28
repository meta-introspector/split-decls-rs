macro_rules! MiniGraph {
    () => {
        # [doc = " Represents the graph of constraints. For each `R1: R2` constraint we create"] # [doc = " an edge `R1 -> R2` in the graph."] struct MiniGraph < 'tcx > { # [doc = " Map from a region to the index of the node in the graph."] nodes : FxIndexMap < ty :: Region < 'tcx > , LeakCheckNode > , # [doc = " Map from node index to SCC, and stores the successors of each SCC. All"] # [doc = " the regions in the same SCC are equal to one another, and if `S1 -> S2`,"] # [doc = " then `S1: S2`."] sccs : Sccs < LeakCheckNode , LeakCheckScc > , }
    };
}

MiniGraph!()