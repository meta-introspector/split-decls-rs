macro_rules! deps {
    () => {
        GraphRef!();
        VisitMap!();
        Bfs!();
        Create!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < N , VM > Bfs < N , VM > where N : Copy + PartialEq , VM : VisitMap < N > , { # [doc = " Create a new **Bfs**, using the graph's visitor map, and put **start**"] # [doc = " in the stack of nodes to visit."] pub fn new < G > (graph : G , start : N) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { let mut discovered = graph . visit_map () ; discovered . visit (start) ; let mut stack = VecDeque :: new () ; stack . push_front (start) ; Bfs { stack , discovered } } # [doc = " Return the next node in the bfs, or **None** if the traversal is done."] pub fn next < G > (& mut self , graph : G) -> Option < N > where G : IntoNeighbors < NodeId = N > , { if let Some (node) = self . stack . pop_front () { for succ in graph . neighbors (node) { if self . discovered . visit (succ) { self . stack . push_back (succ) ; } } return Some (node) ; } None } }
    };
}

impl_50!();