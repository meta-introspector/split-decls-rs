macro_rules! deps {
    () => {
        Graph!();
        EdgeType!();
        WalkNeighbors!();
        IndexType!();
        NodeIndex!();
        EdgeIndex!();
    };
}

macro_rules! impl_722 {
    () => {
        deps!();
        impl < Ix : IndexType > WalkNeighbors < Ix > { # [doc = " Step to the next edge and its endpoint node in the walk for graph `g`."] # [doc = ""] # [doc = " The next node indices are always the others than the starting point"] # [doc = " where the `WalkNeighbors` value was created."] # [doc = " For an `Outgoing` walk, the target nodes,"] # [doc = " for an `Incoming` walk, the source nodes of the edge."] pub fn next < N , E , Ty : EdgeType > (& mut self , g : & Graph < N , E , Ty , Ix > ,) -> Option < (EdgeIndex < Ix > , NodeIndex < Ix >) > { match g . edges . get (self . next [0] . index ()) { None => { } Some (edge) => { let ed = self . next [0] ; self . next [0] = edge . next [0] ; return Some ((ed , edge . node [1])) ; } } while let Some (edge) = g . edges . get (self . next [1] . index ()) { let ed = self . next [1] ; self . next [1] = edge . next [1] ; if edge . node [0] != self . skip_start { return Some ((ed , edge . node [0])) ; } } None } pub fn next_node < N , E , Ty : EdgeType > (& mut self , g : & Graph < N , E , Ty , Ix > ,) -> Option < NodeIndex < Ix > > { self . next (g) . map (| t | t . 1) } pub fn next_edge < N , E , Ty : EdgeType > (& mut self , g : & Graph < N , E , Ty , Ix > ,) -> Option < EdgeIndex < Ix > > { self . next (g) . map (| t | t . 0) } }
    };
}

impl_722!()